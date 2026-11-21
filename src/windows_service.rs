//! Running `hbbs` and `hbbr` as Windows services.
//!
//! The binaries stay ordinary console programs. The Windows service manager (SCM) starts a service by running the
//! executable with the arguments stored in the service, so the service is declared with the flag
//! `--windows-service` (see `docs/windows.md`):
//!
//! ```text
//! sc.exe create SCTGDeskHbbs binPath= "\"C:\Program Files\SCTGDesk\hbbs.exe\" --windows-service" start= auto
//! ```
//!
//! With the flag the program (1) moves to its data directory, because the SCM starts services in
//! `C:\Windows\System32` and the servers keep their key pair, database and `.env` in the working directory,
//! (2) logs to files, because a service has no console, and (3) talks to the SCM so that `sc stop` and a shutdown of
//! the machine stop it cleanly.
//!
//! Everything here is a no-op on the other platforms: [`requested`] is always false there.

use hbb_common::ResultType;
use std::path::PathBuf;

/// The flag that `sc.exe create` puts in the command line of the service.
pub const FLAG: &str = "--windows-service";

/// True when the process was started as a Windows service (the flag is on the command line).
/// Always false on the other platforms, where the flag is refused.
pub fn requested() -> bool {
    cfg!(windows) && std::env::args().any(|a| a == FLAG)
}

/// The folder where a service keeps its files: `SCTGDESK_DATA_DIR` when set (system-wide environment variable),
/// else `%ProgramData%\SCTGDesk`. Both servers of a machine share it, like `/var/lib/rustdesk-server` on Linux.
pub fn data_dir() -> PathBuf {
    if let Some(dir) = std::env::var_os("SCTGDESK_DATA_DIR").filter(|d| !d.is_empty()) {
        return PathBuf::from(dir);
    }
    let base = std::env::var_os("ProgramData").unwrap_or_else(|| r"C:\ProgramData".into());
    PathBuf::from(base).join("SCTGDesk")
}

/// The folder for the log files of a service, `None` when the program does not run as a service (it then logs to
/// its console).
pub fn log_dir() -> Option<PathBuf> {
    requested().then(|| data_dir().join("logs"))
}

/// Runs `main` as the body of the Windows service `name`.
///
/// Blocks until the service stops. `main` is the normal `main` of the program: it blocks while the server runs.
///
/// # Errors
/// When the program was not started by the service manager (the flag was typed in a console) or, off Windows,
/// always.
#[cfg(windows)]
pub fn run(name: &'static str, main: fn() -> ResultType<()>) -> ResultType<()> {
    imp::run(name, main)
}

/// See the Windows version: services exist only there.
#[cfg(not(windows))]
pub fn run(_name: &'static str, _main: fn() -> ResultType<()>) -> ResultType<()> {
    hbb_common::bail!("{FLAG} is only available on Windows")
}

#[cfg(windows)]
mod imp {
    use super::{data_dir, ResultType};
    use std::{
        ffi::OsString,
        sync::{Mutex, OnceLock},
        time::Duration,
    };
    use windows_service::{
        define_windows_service,
        service::{
            ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
            ServiceType,
        },
        service_control_handler::{self, ServiceControlHandlerResult, ServiceStatusHandle},
        service_dispatcher,
    };

    /// The program to run, set before the dispatcher starts: the FFI entry point has a fixed signature and cannot
    /// receive it as a parameter.
    static BODY: OnceLock<(&'static str, fn() -> ResultType<()>)> = OnceLock::new();
    /// Needed by the control handler, which runs on another thread, to report the final state.
    static STATUS: Mutex<Option<ServiceStatusHandle>> = Mutex::new(None);

    define_windows_service!(ffi_service_main, service_main);

    pub fn run(name: &'static str, main: fn() -> ResultType<()>) -> ResultType<()> {
        BODY.set((name, main)).ok();
        service_dispatcher::start(name, ffi_service_main).map_err(|e| {
            // ERROR_FAILED_SERVICE_CONTROLLER_CONNECT: the program was started from a console and not by the service
            // manager, the usual mistake. Its raw text ("IO error in winapi call") says nothing to the user.
            const ERROR_FAILED_SERVICE_CONTROLLER_CONNECT: i32 = 1063;
            let from_console = matches!(
                &e,
                windows_service::Error::Winapi(io)
                    if io.raw_os_error() == Some(ERROR_FAILED_SERVICE_CONTROLLER_CONNECT)
            );
            if from_console {
                hbb_common::anyhow::anyhow!(
                    "{} is meant for the Windows service manager (it is set by `sc.exe create`, see docs/windows.md): \
                     to run {name} in this console, leave it out",
                    super::FLAG
                )
            } else {
                hbb_common::anyhow::anyhow!("cannot connect to the Windows service manager: {e}")
            }
        })?;
        Ok(())
    }

    fn report(state: ServiceState, exit: ServiceExitCode, accepted: ServiceControlAccept) {
        if let Some(handle) = *STATUS.lock().unwrap_or_else(|e| e.into_inner()) {
            let _ = handle.set_service_status(ServiceStatus {
                service_type: ServiceType::OWN_PROCESS,
                current_state: state,
                controls_accepted: accepted,
                exit_code: exit,
                checkpoint: 0,
                wait_hint: Duration::default(),
                process_id: None,
            });
        }
    }

    fn service_main(_arguments: Vec<OsString>) {
        let Some(&(name, main)) = BODY.get() else {
            return;
        };
        // The servers have no graceful shutdown: on a stop request the state is reported and the process ends.
        let handle = service_control_handler::register(name, |event| match event {
            ServiceControl::Stop | ServiceControl::Shutdown => {
                report(
                    ServiceState::Stopped,
                    ServiceExitCode::Win32(0),
                    ServiceControlAccept::empty(),
                );
                std::process::exit(0);
            }
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            _ => ServiceControlHandlerResult::NotImplemented,
        });
        let Ok(handle) = handle else {
            return;
        };
        *STATUS.lock().unwrap_or_else(|e| e.into_inner()) = Some(handle);

        // Services start in C:\Windows\System32: keep the key pair, the database and `.env` in the data directory.
        let dir = data_dir();
        let prepared =
            std::fs::create_dir_all(dir.join("logs")).and_then(|_| std::env::set_current_dir(&dir));
        if prepared.is_err() {
            report(
                ServiceState::Stopped,
                ServiceExitCode::ServiceSpecific(2),
                ServiceControlAccept::empty(),
            );
            return;
        }
        report(
            ServiceState::Running,
            ServiceExitCode::Win32(0),
            ServiceControlAccept::STOP | ServiceControlAccept::SHUTDOWN,
        );
        // Blocks while the server runs; returning means it failed or ended on its own.
        let result = main();
        if let Err(e) = &result {
            hbb_common::log::error!("{name} stopped: {e}");
        }
        report(
            ServiceState::Stopped,
            if result.is_ok() {
                ServiceExitCode::Win32(0)
            } else {
                ServiceExitCode::ServiceSpecific(1)
            },
            ServiceControlAccept::empty(),
        );
    }
}
