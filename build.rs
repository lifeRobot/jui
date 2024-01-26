use std::{env, fs};
use std::path::PathBuf;
use jui_file::WriteToFile;

include!("version.rs");

/// this build.rs just generate included_library.rs<br />
/// support automatic import of widgets to slint library path
pub fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR does not exist");
    let out_dir = PathBuf::from(out_dir).join("jui").join("build");
    version_file.set(out_dir.join("version"));// change version file

    // load and check version
    // if version is not exists or read version fail or ne file_version, generate included_library.rs and lock version
    not_exists_exec!(return generate_included_library_lock_version());
    let file_version = read_to_str_fail_exec!(return generate_included_library_lock_version());
    ne_version_exec!(file_version,return generate_included_library_lock_version());

    generate_included_library();
}

/// generate included_library.rs and lock version
fn generate_included_library_lock_version() {
    generate_included_library();
    lock_version();
}

/// generate included_library.rs
fn generate_included_library() {
    // build included_library path
    let lib_dir = env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is None");
    let lib_dir = PathBuf::from(lib_dir);
    let mut included_library = lib_dir.join("included_library");
    included_library.set_extension("rs");

    // build included content
    let mut included_context = build_start_context();
    add_widgets(&mut included_context, lib_dir.join("widgets"));
    included_context.pop();// remove last comma
    push_end_context(&mut included_context);

    included_library.try_write_to_file(included_context).expect("generate included_library.rs fail");
}

/// build included_library.rs start str
fn build_start_context() -> String {
    r#"/// auto generate by build.rs
use jui_file::WriteToFile;

fn jui_path_push_widgets(mut jui_path: PathBuf, widgets: &str) -> PathBuf {
    jui_path.push(widgets);
    jui_path
}

pub fn included_library(jui_path: PathBuf) {
    let widgets_list = vec!["#.to_string()
}

/// push end str to included_library.rs
fn push_end_context(included_context: &mut String) {
    let end_context = r#"
    ];

    for (path, bytes) in widgets_list {
        path.try_write_to_file(&bytes).expect(&format!("write slint file to {path:?} fail"));
    }
}"#;
    included_context.push_str(end_context);
}

/// add widgets and out_dir path to vec
fn add_widgets(included_context: &mut String, widgets_dir: PathBuf) {
    for dir in fs::read_dir(widgets_dir).expect("read widgets fail").filter_map(Result::ok) {
        let path = dir.path();
        if path.is_dir() {
            add_widgets(included_context, path);
            continue;
        }

        // build widgets and out_dir correspondence
        let sub_path = get_widgets_sub_path(&path);
        // let include_path = sub_path.replace('\\', "/");// change windows dir separator
        included_context.push_str(&format!("\n        (jui_path_push_widgets(jui_path.clone(), {sub_path:?}), include_bytes!({sub_path:?}).to_vec()),"));
    }
}

/// get sub path
fn get_widgets_sub_path(path: &PathBuf) -> String {
    let path = path.display().to_string();
    let index = path.find("widgets\\").expect(&format!("the path[{path:?}] not contain widgets\\"));
    path[index..].to_string()
}
