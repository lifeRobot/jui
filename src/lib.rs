#[cfg(not(feature = "surrealism_ui"))]
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use slint_build::{CompileError, CompilerConfiguration};
pub use jui_file;

#[cfg(feature = "surrealism_ui")]
pub mod surrealism_ui;

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
    let jui_widgets = env!("JUI_WIDGETS", "JUI_WIDGETS is None");
    let jui_widgets = PathBuf::from(jui_widgets);

    let mut library_paths = {
        #[cfg(feature = "surrealism_ui")] {
            surrealism_ui::SurrealismUI::default().build_library_path()
        }
        #[cfg(not(feature = "surrealism_ui"))] HashMap::new()
    };

    library_paths.insert(alias, jui_widgets);
    let config = CompilerConfiguration::new().with_library_paths(library_paths);
    slint_build::compile_with_config(path, config)
}

/// Compile the .slint file and generate rust code for it.<br />
/// custom surrealism_ui config
#[cfg(feature = "surrealism_ui")]
pub fn compile_with_surrealism_ui(path: impl AsRef<Path>, surrealism: surrealism_ui::SurrealismUI) -> Result<(), CompileError> {
    compile_alias_with_surrealism_ui(path, "jui".into(), surrealism)
}

/// Compile the .slint file and generate rust code for it.<br />
/// custom surrealism_ui config and add an alias for jui
#[cfg(feature = "surrealism_ui")]
pub fn compile_alias_with_surrealism_ui(path: impl AsRef<Path>, alias: String, surrealism: surrealism_ui::SurrealismUI) -> Result<(), CompileError> {
    let jui_widgets = env!("JUI_WIDGETS", "JUI_WIDGETS is None");
    let jui_widgets = PathBuf::from(jui_widgets);

    // add library to slint
    let mut library_paths = surrealism.build_library_path();
    library_paths.insert(alias, jui_widgets);
    let config = CompilerConfiguration::new().with_library_paths(library_paths);
    slint_build::compile_with_config(path, config)
}
