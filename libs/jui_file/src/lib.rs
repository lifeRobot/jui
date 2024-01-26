use std::{fs, io};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use cbsk_base::log;
pub use write::*;

pub mod write;

/// directory separator
#[allow(non_upper_case_globals)]
pub static separator: char = {
    #[cfg(windows)] { '\\' }
    #[cfg(not(windows))] '/'
};

/// directory separator
#[allow(non_upper_case_globals)]
pub static separator_str: &str = {
    #[cfg(windows)] { "\\" }
    #[cfg(not(windows))] "/"
};

/// try manipulate file fail log
macro_rules! err_log {
    ($f:ident($file:expr),$name:expr) => {
        if let Err(e) = $f($file) {
            log::error!("{}[{:?}] fail: {e:?}",$name,$file);
        }
    };
}

/// create dir if does not exists<br />
/// if dir is file, will be create parent dir<br />
/// if create fail, will be call log::error<br />
/// see [log::error], [try_create_dir]
pub fn create_dir(dir: &Path) {
    err_log!(try_create_dir(dir),"create dir");
}

/// try create dir if does not exists<br />
/// if dir is file, will be create parent dir
pub fn try_create_dir(dir: &Path) -> io::Result<()> {
    if dir.exists() {
        return Ok(());
    }
    just_create_dir(dir)
}

/// create file if does not exists<br />
/// if create fail, will be call log::error<br />
/// see [log::error], [try_create_file]<br />
/// if y want to create or open file, see [open_create_file]
pub fn create_file(file: &Path) {
    err_log!(try_create_file(file),"create file");
}

/// try create file if does not exists<br />
/// if file is dir, will be return Err<br />
/// if y want to create or open file, see [open_create_file]
pub fn try_create_file(file: &Path) -> io::Result<()> {
    if file.exists() {
        return Ok(());
    }
    if file.is_dir() {
        return Err(io::Error::other(format!("{file:?} is a directory")));
    }

    just_create_parent_dir(file)?;
    File::create(file)?;
    Ok(())
}

/// just create parent dir, not check exists
fn just_create_parent_dir(dir: &Path) -> io::Result<()> {
    // not exists and not a dir
    // if get parent dir fail, maybe dir is root dir, return Ok
    // maybe have bug
    let parent = cbsk_base::match_some_return!(dir.parent(),Ok(()));
    fs::create_dir_all(parent)
}

/// just create dir, not check exists
fn just_create_dir(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        return fs::create_dir_all(dir);
    }

    just_create_parent_dir(dir)
}

/// read all all to vec<br />
/// if file is so big, call this method is not good idea<br />
/// if read fail, will be call log::error and return Vec::new()<br />
/// see [log::error], [try_read_to_vec]
pub fn read_to_vec(file: &Path) -> Vec<u8> {
    try_read_to_vec(file).unwrap_or_else(|e| {
        log::error!("read file[{file:?}] to vec fail: {e:?}");
        Vec::new()
    })
}

/// try read all bytes to vec<br />
/// if file is so big, call this method is not good idea
pub fn try_read_to_vec(file: &Path) -> io::Result<Vec<u8>> {
    let mut file = File::open(file)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

/// read all bytes to string<br />
/// if file is so big, call this method is not good idea
/// if read fail, will be call log::error and return String::new()<br />
/// see [log::error], [try_read_to_str]
pub fn read_to_str(file: &Path) -> String {
    try_read_to_str(file).unwrap_or_else(|e| {
        log::error!("read file[{file:?}] to string fail: {e:?}");
        String::new()
    })
}

/// try read all bytes to string<br />
/// if file is so big, call this method is not good idea
pub fn try_read_to_str(file: &Path) -> io::Result<String> {
    let mut file = File::open(file)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

/// will be delete file and create file
pub fn recreate_file(path: &Path) -> io::Result<File> {
    if path.exists() {
        fs::remove_file(path)?;
    }

    just_create_parent_dir(path)?;
    File::create(path)
}

/// open or create file
pub fn open_create_file(path: &Path) -> io::Result<File> {
    if path.exists() {
        return File::options().read(true).write(true).append(true).open(path);
    }

    just_create_parent_dir(path)?;
    File::create(path)
}
