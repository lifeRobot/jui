use std::collections::HashMap;
use std::{env, fs};
use std::path::{Path, PathBuf};
use slint_build::{CompileError, CompilerConfiguration};

/// Compile the .slint file and generate rust code for it.
pub fn compile(path: impl AsRef<Path>) -> Result<(), CompileError> {
    let version = "0.1.1";
    let jui_path = get_jui_path();
    let version_file = jui_path.clone().join("version");

    // if version is not exists, copy file to dir
    if !version_file.exists() {
        copy_slint_file(jui_path.clone());
        lock_version(&version_file, version);
        return add_slint_library_path(jui_path, path);
    }

    // load and check version
    let file_version = jui_file::try_read_to_str(&version_file);
    let file_version =
        match file_version {
            Ok(file_version) => { file_version }
            Err(_) => {
                // read version fail, copy file and lock version
                copy_slint_file(jui_path.clone());
                lock_version(&version_file, version);
                return add_slint_library_path(jui_path, path);
            }
        };

    // check version, if version is ne, copy file and lock version
    if file_version.ne(version) {
        copy_slint_file(jui_path.clone());
        lock_version(&version_file, version);
        return add_slint_library_path(jui_path, path);
    }

    // just add library path
    add_slint_library_path(jui_path, path)
}

/// add jui_path to slint library
fn add_slint_library_path(jui_path: PathBuf, path: impl AsRef<Path>) -> Result<(), CompileError> {
    let library_paths = HashMap::from([("jui".to_string(), jui_path.join("widgets"))]);
    let config = CompilerConfiguration::new().with_library_paths(library_paths);
    slint_build::compile_with_config(path, config)
}

/// get jui out dir path
fn get_jui_path() -> PathBuf {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR does not exist");
    PathBuf::from(out_dir).join("jui")
}

/// lock version to file
fn lock_version(version_file: &PathBuf, version: &str) {
    if version_file.exists() {
        let _ = fs::remove_file(version_file);
    }

    jui_file::try_write_to_file(version_file, version.as_bytes()).expect(&format!("write version file to {version_file:?} fail"));
}

/// copy slint file to jui path(out dir)
fn copy_slint_file(jui_path: PathBuf) {
    // fs::read_dir will be fail, so currently, only [include-bytes] can be used
    // load all slint file
    let button = include_bytes!("../widgets/button.slint");
    // input
    let input_base = include_bytes!("../widgets/input/input_base.slint");
    let input = include_bytes!("../widgets/input/input.slint");
    let underline_input = include_bytes!("../widgets/input/underline_input.slint");

    // build slint path
    let jui_path = jui_path.join("widgets");
    let mut button_path = jui_path.join("button");
    let mut input_base_path = jui_path.join("input").join("input_base");
    let mut input_path = jui_path.join("input").join("input");
    let mut underline_input_path = jui_path.join("input").join("underline_input");

    // set suffix
    button_path.set_extension("slint");
    input_base_path.set_extension("slint");
    input_path.set_extension("slint");
    underline_input_path.set_extension("slint");

    // if file exists, remove file
    // if not remove, write file will be fail
    if button_path.exists() {
        let _ = fs::remove_file(&button_path);
    }
    if input_base_path.exists() {
        let _ = fs::remove_file(&input_base_path);
    }
    if input_path.exists() {
        let _ = fs::remove_file(&input_path);
    }
    if underline_input_path.exists() {
        let _ = fs::remove_file(&underline_input_path);
    }

    jui_file::try_write_to_file(&button_path, button).expect(&format!("write slint file to {button_path:?} fail"));
    jui_file::try_write_to_file(&input_base_path, input_base).expect(&format!("write slint file to {input_base_path:?} fail"));
    jui_file::try_write_to_file(&input_path, input).expect(&format!("write slint file to {input_path:?} fail"));
    jui_file::try_write_to_file(&underline_input_path, underline_input).expect(&format!("write slint file to {underline_input_path:?} fail"));
}
