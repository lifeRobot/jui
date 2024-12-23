use std::fmt::{Debug, Display, Formatter};

/// root dir extracted from SurrealismUI zip file
pub const EXTRACT_NAME: &str = "v0.5.3";
// pub const EXTRACT_NAME: &str = "surrealism-ui_0.5.3";
/// download SurrealismUI by github url
pub const GITHUB: &str =
    "https://github.com/Surrealism-All/SurrealismUI/releases/download/v0.5.3/v0.5.3.zip";
// pub const GITHUB: &str = "https://github.com/Surrealism-All/SurrealismUI/releases/download/v0.5.1/surrealism-ui_0.5.1.zip";
/// download SurrealismUI by gitee url
pub const GITEE: &str =
    "https://gitee.com/life_robot/SurrealismUI/releases/download/v0.5.3/v0.5.3.zip";
// pub const GITEE: &str = "https://gitee.com/life_robot/SurrealismUI/releases/download/v0.5.1/surrealism-ui_0.5.1.zip";

/// SurrealismUI download url
pub enum SurrealismUrl {
    /// download SurrealismUI by github
    GITHUB,
    /// download SurrealismUI by gitee
    GITEE,
}

/// custom method
impl SurrealismUrl {
    /// get download url
    pub fn get_url(&self) -> &'static str {
        match self {
            SurrealismUrl::GITHUB => GITHUB,
            SurrealismUrl::GITEE => GITEE,
        }
    }
}

/// support debug
impl Debug for SurrealismUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_url())
    }
}

/// support display
impl Display for SurrealismUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_url())
    }
}
