use std::{fs, io};
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use cbsk_base::log;

/// write to file
pub trait WriteToFile: Debug {
    /// write bytes to file<br />
    /// if file is not exists, will create file and write bytes<br />
    /// if write fail, will be call log::error<br />
    /// see [log::error], [Self::try_write_to_file]
    fn write_to_file(&self, bytes: impl AsRef<[u8]>) {
        if let Err(e) = self.try_write_to_file(bytes) {
            log::error!("write bytes to file[{self:?}] fail: {e:?}");
        }
    }

    /// try write bytes to file<br />
    /// if file is not exists, will create file and write bytes
    fn try_write_to_file(&self, bytes: impl AsRef<[u8]>) -> io::Result<()>;
}

/// support Path write_to_file
impl WriteToFile for Path {
    /// try write bytes to file<br />
    /// if file is not exists, will create file and write bytes<br />
    /// will entirely replace its contents if it does, see [fs::write]
    fn try_write_to_file(&self, bytes: impl AsRef<[u8]>) -> io::Result<()> {
        super::try_create_file(self)?;
        fs::write(self, bytes)
    }
}

/// support PathBuf write_to_file
impl WriteToFile for PathBuf {
    /// try write bytes to file<br />
    /// if file is not exists, will create file and write bytes<br />
    /// will entirely replace its contents if it does, see [fs::write]
    fn try_write_to_file(&self, bytes: impl AsRef<[u8]>) -> io::Result<()> {
        self.as_path().try_write_to_file(bytes)
    }
}

/// support String write_to_file
impl WriteToFile for String {
    /// try write bytes to file<br />
    /// if file is not exists, will create file and write bytes<br />
    /// will entirely replace its contents if it does, see [fs::write]
    fn try_write_to_file(&self, bytes: impl AsRef<[u8]>) -> io::Result<()> {
        Path::new(self).try_write_to_file(bytes)
    }
}

/// support &str write_to_file
impl WriteToFile for &str {
    /// try write bytes to file<br />
    /// if file is not exists, will create file and write bytes<br />
    /// will entirely replace its contents if it does, see [fs::write]
    fn try_write_to_file(&self, bytes: impl AsRef<[u8]>) -> io::Result<()> {
        Path::new(self).try_write_to_file(bytes)
    }
}
