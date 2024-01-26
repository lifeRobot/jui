/// auto generate by build.rs
use jui_file::WriteToFile;

fn jui_path_push_widgets(mut jui_path: PathBuf, widgets: &str) -> PathBuf {
    jui_path.push(widgets);
    jui_path
}

pub fn included_library(jui_path: PathBuf) {
    let widgets_list = vec![
        (jui_path_push_widgets(jui_path.clone(), "widgets\\button.slint"), include_bytes!("widgets\\button.slint").to_vec()),
        (jui_path_push_widgets(jui_path.clone(), "widgets\\input\\border_input.slint"), include_bytes!("widgets\\input\\border_input.slint").to_vec()),
        (jui_path_push_widgets(jui_path.clone(), "widgets\\input\\input.slint"), include_bytes!("widgets\\input\\input.slint").to_vec()),
        (jui_path_push_widgets(jui_path.clone(), "widgets\\input\\input_base.slint"), include_bytes!("widgets\\input\\input_base.slint").to_vec()),
        (jui_path_push_widgets(jui_path.clone(), "widgets\\input\\underline_input.slint"), include_bytes!("widgets\\input\\underline_input.slint").to_vec())
    ];

    for (path, bytes) in widgets_list {
        path.try_write_to_file(&bytes).expect(&format!("write slint file to {path:?} fail"));
    }
}