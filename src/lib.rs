#[cfg(not(feature = "surrealism_ui"))]
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
pub use slint_build;
use slint_build::CompileError;
pub use cbsk_file;
use crate::jui_config::JuiConfig;

#[cfg(feature = "surrealism_ui")]
pub mod surrealism_ui;
pub mod jui_config;

/// Compile the .slint file and generate rust code for it.
pub fn compile(path: impl AsRef<Path>) -> Result<(), CompileError> {
    compile_with_jui_config(path, JuiConfig::default())
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
    compile_with_jui_config(path, JuiConfig::default().set_alias(alias))
}

/// Compile the .slint file and generate rust code for it.<br />
/// custom surrealism_ui config
#[cfg(feature = "surrealism_ui")]
pub fn compile_with_surrealism_ui(path: impl AsRef<Path>, surrealism: surrealism_ui::SurrealismUI) -> Result<(), CompileError> {
    compile_with_jui_config(path, JuiConfig::default().set_surrealism_ui(surrealism))
}

/// Compile the .slint file and generate rust code for it.<br />
/// custom surrealism_ui config and add an alias for jui
#[cfg(feature = "surrealism_ui")]
pub fn compile_alias_with_surrealism_ui(path: impl AsRef<Path>, alias: String, surrealism: surrealism_ui::SurrealismUI) -> Result<(), CompileError> {
    let jui_conf = JuiConfig::default().set_alias(alias).set_surrealism_ui(surrealism);
    compile_with_jui_config(path, jui_conf)
}

/// Compile the .slint file and generate rust code with custom jui config
pub fn compile_with_jui_config(path: impl AsRef<Path>, jui_config: JuiConfig) -> Result<(), CompileError> {
    let jui_widgets = env!("JUI_WIDGETS", "JUI_WIDGETS is None");
    let jui_widgets = PathBuf::from(jui_widgets);

    let library_path = {
        #[cfg(feature = "surrealism_ui")] {
            let mut library_path = jui_config.surrealism_ui.build_library_path();
            library_path.insert(jui_config.alias, jui_widgets);
            library_path
        }
        #[cfg(not(feature = "surrealism_ui"))] HashMap::from([(jui_config.alias, jui_widgets)])
    };

    let config = jui_config.slint_confg.with_library_paths(library_path);
    slint_build::compile_with_config(path, config)
}
