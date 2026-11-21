# Running the server on Windows

`hbbs` and `hbbr` are console programs. Started with `--windows-service` they also behave as **Windows services**: they
move to their data directory, log to files and stop cleanly when the service manager asks. The services are declared
with `sc.exe`, there is no installer to run.

## Install the programs

With winget (the package keeps the three programs on the `PATH`; `--scope machine` puts them where a service can reach
them):

```powershell
winget install SCTG.SCTGDeskServer --scope machine
```

Or unzip `sctgdesk-server-<version>-windows-x86_64.zip` (or `-aarch64`) from the
[releases](https://github.com/sctg-development/sctgdesk-server/releases) to a folder such as
`C:\Program Files\SCTGDesk`.

## Create the services

In an **administrator** PowerShell. Replace the paths if you unzipped the archive yourself; `Get-Command` finds the
winget ones.

```powershell
$hbbs = (Get-Command hbbs).Source
$hbbr = (Get-Command hbbr).Source

# note the space after "binPath=" and after "start=": sc.exe requires it
sc.exe create SCTGDeskHbbr binPath= "`"$hbbr`" --windows-service" start= auto DisplayName= "SCTGDesk relay server (hbbr)"
sc.exe create SCTGDeskHbbs binPath= "`"$hbbs`" --windows-service -r relay.example.com" start= auto depend= SCTGDeskHbbr DisplayName= "SCTGDesk signal server (hbbs)"

# restart by itself after a crash (after 10 s, forever)
sc.exe failure SCTGDeskHbbr reset= 86400 actions= restart/10000
sc.exe failure SCTGDeskHbbs reset= 86400 actions= restart/10000

sc.exe start SCTGDeskHbbr
sc.exe start SCTGDeskHbbs
```

Everything after `--windows-service` in `binPath=` is a normal option of the program (`-r`, `-k`, `--logged-in-only`,
`--api-port`, ...: see `hbbs --help`). To change them: `sc.exe config SCTGDeskHbbs binPath= "..."` then restart.

Open the ports in the firewall (21114 API and console, 21115, 21116 TCP and UDP, 21117, 21118, 21119):

```powershell
New-NetFirewallRule -DisplayName "SCTGDesk" -Direction Inbound -Protocol TCP -LocalPort 21114-21119 -Action Allow
New-NetFirewallRule -DisplayName "SCTGDesk (UDP)" -Direction Inbound -Protocol UDP -LocalPort 21116 -Action Allow
```

## Where the files are

| What | Where |
|---|---|
| Key pair (`id_ed25519*`), database, secrets, `oauth2.toml`, `s3config.toml` | `%ProgramData%\SCTGDesk` |
| Logs (rotated at 10 MB, 5 kept) | `%ProgramData%\SCTGDesk\logs\hbbs*.log`, `hbbr*.log` |
| Environment variables of the programs | a `.env` file in `%ProgramData%\SCTGDesk` (`KEY=value` lines, for example `OAUTH2_CREATE_USER=1`) |

Set the system environment variable `SCTGDESK_DATA_DIR` (and restart the services) to use another folder. Both services
share the folder. Back it up: it holds the private key.

## Stop and remove

```powershell
sc.exe stop SCTGDeskHbbs ; sc.exe stop SCTGDeskHbbr
sc.exe delete SCTGDeskHbbs ; sc.exe delete SCTGDeskHbbr
```

The data folder is never deleted.

## Notes

- The flag only works when the service manager starts the program. Typed in a console it fails with "not started by the
  Windows service manager": to run the server by hand, leave it out.
- The services run as `LocalSystem` by default. For a less privileged account, use `sc.exe config ... obj=` and give the
  account write access to the data folder.
- The Windows binaries are not code-signed: SmartScreen or an antivirus may warn on the first run.
