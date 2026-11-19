// NeoSymbian (target_os = "symbian"): temporary files with std alone (no rustix). The Symbian file server can't
// delete an open file, so an "unnamed" temporary file is a named one that stays until the next clean-up of the
// temporary directory.
// shortcut: unnamed temp files are not unlinked while open, upgrade when a delete-on-close open exists in P.I.P.S.
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::path::Path;

use crate::util;

pub fn create_named(
    path: &Path,
    open_options: &mut OpenOptions,
    _permissions: Option<&std::fs::Permissions>,
) -> io::Result<File> {
    open_options.read(true).write(true).create_new(true).open(path)
}

pub fn create(dir: &Path) -> io::Result<File> {
    util::create_helper(
        dir,
        OsStr::new(".tmp"),
        OsStr::new(""),
        crate::NUM_RAND_CHARS,
        |path| {
            let f = OpenOptions::new().read(true).write(true).create_new(true).open(&path)?;
            // where the platform allows it, the name goes now
            let _ = fs::remove_file(&path);
            Ok(f)
        },
    )
}

pub fn reopen(_file: &File, path: &Path) -> io::Result<File> {
    OpenOptions::new().read(true).write(true).open(path)
}

pub fn persist(old_path: &Path, new_path: &Path, overwrite: bool) -> io::Result<()> {
    if new_path.exists() {
        if !overwrite {
            return Err(io::ErrorKind::AlreadyExists.into());
        }
        // Symbian's rename never replaces a file
        fs::remove_file(new_path)?;
    }
    fs::rename(old_path, new_path)
}

pub fn keep(_: &Path) -> io::Result<()> {
    Ok(())
}
