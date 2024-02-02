use slint_build::CompilerConfiguration;
#[cfg(feature = "surrealism_ui")]
use crate::surrealism_ui::SurrealismUI;

/// jui build config
pub struct JuiConfig {
    /// jui alias, default is jui
    pub(crate) alias: String,
    /// slint config
    pub(crate) slint_confg: CompilerConfiguration,
    /// surrealism_ui config
    #[cfg(feature = "surrealism_ui")]
    pub(crate) surrealism_ui: SurrealismUI,
}

/// custom method
impl JuiConfig {
    /// set jui alias
    pub fn set_alias(mut self, alias: impl Into<String>) -> Self {
        self.alias = alias.into();
        self
    }

    /// set slint config, support custom slint config
    pub fn set_slint_config(mut self, slint_confg: CompilerConfiguration) -> Self {
        self.slint_confg = slint_confg;
        self
    }

    /// set surrealism_ui<br />
    /// see [SurrealismUI]
    #[cfg(feature = "surrealism_ui")]
    pub fn set_surrealism_ui(mut self, surrealism_ui: SurrealismUI) -> Self {
        self.surrealism_ui = surrealism_ui;
        self
    }
}

/// support default
impl Default for JuiConfig {
    fn default() -> Self {
        Self {
            alias: "jui".into(),
            slint_confg: CompilerConfiguration::default(),
            #[cfg(feature = "surrealism_ui")]
            surrealism_ui: SurrealismUI::default(),
        }
    }
}
