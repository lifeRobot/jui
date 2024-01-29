use std::env;
use std::path::PathBuf;

/// support automatic import of widgets to slint library path
pub fn main() {
    // set JUI_WIDGETS
    let lib_dir = env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is None");
    let out_widget_dir = PathBuf::from(lib_dir).join("widgets");

    println!("cargo:rustc-env=JUI_WIDGETS={}", out_widget_dir.display());
}
