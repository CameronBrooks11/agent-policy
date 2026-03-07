// Filesystem helpers — Phase 2

use std::{fs, io, path::Path};

use crate::error::{Error, Result};

/// Write `content` to `path` atomically.
///
/// Creates parent directories if they do not exist.
/// Writes to a temporary file alongside `path`, then renames atomically.
///
/// # Errors
///
/// Returns [`Error::Io`] if any filesystem operation fails.
pub fn write_atomic(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| Error::Io {
            path: parent.to_owned(),
            source: e,
        })?;
    }

    let tmp = path.with_extension("agent-policy.tmp");
    fs::write(&tmp, content).map_err(|e| Error::Io {
        path: tmp.clone(),
        source: e,
    })?;

    fs::rename(&tmp, path).map_err(|e| Error::Io {
        path: path.to_owned(),
        source: e,
    })?;

    Ok(())
}

/// Read a file to string, returning `None` if the file does not exist.
///
/// # Errors
///
/// Returns [`Error::Io`] for any error other than "file not found".
#[allow(dead_code)] // used in Phase 3 (check command)
pub fn read_if_exists(path: &Path) -> Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(s) => Ok(Some(s)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(Error::Io {
            path: path.to_owned(),
            source: e,
        }),
    }
}
