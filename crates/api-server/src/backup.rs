//! Backup of the server, downloaded by an administrator when `hbbs` runs with `--allow-remote-backup`.
//!
//! The archive is a `.tar.gz` that holds a consistent copy of the database (`VACUUM INTO`, taken while the server
//! runs), the configuration files (`oauth2.toml`, `s3config.toml`), the key pair (`id_ed25519`,
//! `id_ed25519.pub`) and `secret_key.txt`, with a `MANIFEST.txt`. The files that do not exist are skipped. It holds
//! secrets: the caller has to be an administrator and the answer is not cached.

use std::path::{Path, PathBuf};

use state::ApiState;

/// True when the administrator asked for the download (`--allow-remote-backup` or `ALLOW_REMOTE_BACKUP=Y`).
pub fn is_enabled() -> bool {
    std::env::var("ALLOW_REMOTE_BACKUP").map_or(false, |v| v == "Y")
}

pub struct Backup {
    /// File name proposed to the browser.
    pub filename: String,
    pub bytes: Vec<u8>,
}

fn env_path(var: &str, default: &str) -> PathBuf {
    match std::env::var(var) {
        Ok(v) if !v.is_empty() => PathBuf::from(v),
        _ => PathBuf::from(default),
    }
}

/// The files next to the database that a restoration needs, as `(path)`; their name in the archive is their file name.
fn files_to_save() -> Vec<PathBuf> {
    vec![
        env_path("OAUTH2_CONFIG_FILE", "oauth2.toml"),
        env_path("S3CONFIG_FILE", "s3config.toml"),
        PathBuf::from("id_ed25519"),
        PathBuf::from("id_ed25519.pub"),
        PathBuf::from("secret_key.txt"),
    ]
}

/// A private folder for the copy of the database, deleted when dropped.
struct TempDir(PathBuf);

impl TempDir {
    fn new() -> std::io::Result<Self> {
        let path = std::env::temp_dir().join(format!("sctgdesk-backup-{}", uuid::Uuid::new_v4()));
        let mut builder = std::fs::DirBuilder::new();
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        builder.create(&path)?;
        Ok(Self(path))
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

pub async fn build(state: &ApiState, version: &str) -> Result<Backup, String> {
    let tmp = TempDir::new().map_err(|e| format!("cannot create a temporary folder: {e}"))?;
    let db_copy = tmp.0.join("db_v2.sqlite3");
    state
        .backup_database_to(&db_copy)
        .await
        .map_err(|e| format!("cannot copy the database: {e}"))?;
    let version = version.to_owned();
    tokio::task::spawn_blocking(move || {
        let bytes = archive(&db_copy, &files_to_save(), &version)?;
        drop(tmp);
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Ok(Backup {
            filename: format!("sctgdesk-backup-{secs}.tar.gz"),
            bytes,
        })
    })
    .await
    .map_err(|e| format!("the backup task failed: {e}"))?
}

/// Packs the database copy and the files that exist into a `.tar.gz`, under one folder.
pub fn archive(db_copy: &Path, files: &[PathBuf], version: &str) -> Result<Vec<u8>, String> {
    let io = |e: std::io::Error| e.to_string();
    let gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut tar = tar::Builder::new(gz);
    let mut saved = vec!["db_v2.sqlite3".to_owned()];
    let mut skipped = Vec::new();

    let mut add = |tar: &mut tar::Builder<_>, name: &str, data: &[u8]| -> Result<(), String> {
        let mut header = tar::Header::new_gnu();
        header.set_size(data.len() as u64);
        header.set_mode(0o600);
        header.set_mtime(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        );
        tar.append_data(&mut header, format!("sctgdesk-backup/{name}"), data)
            .map_err(io)
    };

    add(
        &mut tar,
        "db_v2.sqlite3",
        &std::fs::read(db_copy).map_err(io)?,
    )?;
    for path in files {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        match std::fs::read(path) {
            Ok(data) => {
                add(&mut tar, &name, &data)?;
                saved.push(name);
            }
            Err(_) => skipped.push(name),
        }
    }
    let manifest = format!(
        "sctgdesk-server {version}\nfiles: {}\nnot found: {}\n\nTo restore: stop hbbs and hbbr, put the files in their data directory \
         (db_v2.sqlite3 without any -wal or -shm file), mode 600, then start them.\nThis archive holds the private key and the \
         secrets of the server: keep it encrypted and private.\n",
        saved.join(", "),
        if skipped.is_empty() { "-".to_owned() } else { skipped.join(", ") },
    );
    add(&mut tar, "MANIFEST.txt", manifest.as_bytes())?;
    let gz = tar.into_inner().map_err(io)?;
    gz.finish().map_err(io)
}
