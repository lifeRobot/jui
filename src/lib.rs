use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use slint_build::{CompileError, CompilerConfiguration};

include!("../included_library.rs");
include!("../version.rs");

/// Compile the .slint file and generate rust code for it.
pub fn compile(path: impl AsRef<Path>) -> Result<(), CompileError> {
    compile_alias(path, "jui".into())
}

/// Compile the .slint file and generate rust code for it.<br />
/// add an alias for jui<br />
/// ### for example alial="hello"
/// ```slint
/// import { Button } from "@hello/button.slint";
/// ```
/// ### for example allial="jui"
/// ```slint
/// import { Button } from "@jui/button.slint";
/// ```
pub fn compile_alias(path: impl AsRef<Path>, alias: String) -> Result<(), CompileError> {
    let jui_path = get_jui_path();
    version_file.set(jui_path.join("version"));// change version file

    // load and check version
    // if version is not exists or read version fail or ne file_version, copy file and lock version
    not_exists_exec!(return copy_file_lock_version(jui_path,path,alias));
    let file_version = read_to_str_fail_exec!(return copy_file_lock_version(jui_path, path,alias));
    ne_version_exec!(file_version,return copy_file_lock_version(jui_path, path,alias));

    // just add library path
    add_slint_library_path(jui_path, path, alias)
}

/// copy slint file and lock version
fn copy_file_lock_version(jui_path: PathBuf, path: impl AsRef<Path>, alias: String) -> Result<(), CompileError> {
    included_library(jui_path.clone());
    lock_version();
    add_slint_library_path(jui_path, path, alias)
}

/// add jui_path to slint library
fn add_slint_library_path(jui_path: PathBuf, path: impl AsRef<Path>, alias: String) -> Result<(), CompileError> {
    let library_paths = HashMap::from([(alias, jui_path.join("widgets"))]);
    let config = CompilerConfiguration::new().with_library_paths(library_paths);
    slint_build::compile_with_config(path, config)
}

/// get jui out dir path
fn get_jui_path() -> PathBuf {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR does not exist");
    PathBuf::from(out_dir).join("jui")
}
