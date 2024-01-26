use std::io;
use std::io::Write;
use cbsk_base::once_cell::sync::Lazy;
use cbsk_mut_data::mut_data_obj::MutDataObj;

/// lock version
#[allow(non_upper_case_globals)]
static version: &str = "0.1.1";

/// version_file, default is PathBuf::new()<br />
/// need to change it at least once
#[allow(non_upper_case_globals)]
static version_file: Lazy<MutDataObj<PathBuf>> = Lazy::new(MutDataObj::default);

/// if version file is not exists, execute custom code
macro_rules! not_exists_exec {
    ($exec:expr) => {
        if !version_file.exists() {
            $exec
        }
    };
}

/// read string from version file, execute custom code when read fail<br />
/// return string when read success
macro_rules! read_to_str_fail_exec {
    ($exec:expr) => {{
        let file_version = jui_file::try_read_to_str(&version_file);
        match file_version {
            Ok(file_version) => { file_version }
            Err(_) => {
                // read version fail, execute $exec
                $exec
            }
        }
    }};
}

/// if $version ne version, execute $exec
macro_rules! ne_version_exec {
    ($version:expr,$exec:expr) => {
        if $version.ne(version) {
            $exec
        }
    };
}

/// lock version to file
fn lock_version() {
    try_lock_version().expect(&format!("write version to {version_file:?} fail"));
}


fn try_lock_version() -> io::Result<()> {
    let mut file = jui_file::open_create_file(&version_file)?;
    file.set_len(0)?;
    file.write_all(version.as_bytes())?;
    file.flush()
}
