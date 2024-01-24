use std::collections::HashMap;
use std::{env, fs};
use std::path::{Path, PathBuf};
use slint_build::{CompileError, CompilerConfiguration};

/// Compile the .slint file and generate rust code for it.
pub fn compile(path: impl AsRef<Path>) -> Result<(), CompileError> {
    // load all slint file
    let index = include_bytes!("../widgets/index.slint");

    // write slint file to OUT_DIR
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR does not exist");
    let mut path_buf = PathBuf::from(out_dir).join("jui").join("widgets").join("index");
    path_buf.set_extension("slint");
    let _ = fs::remove_file(&path_buf);
    jui_file::try_write_to_file(&path_buf,index).expect(&format!("write slint file to {path_buf:?} fail"));

    // load library paths and .slint path
    let library_paths = HashMap::from([("jui".to_string(), path_buf)]);
    let config = CompilerConfiguration::new().with_library_paths(library_paths);
    slint_build::compile_with_config(path, config)
}