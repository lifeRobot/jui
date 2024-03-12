use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use cbsk_base::anyhow;
use crate::surrealism_ui::surrealism_url::SurrealismUrl;

pub mod surrealism_url;

/// surrealism ui config
pub struct SurrealismUI {
    /// SurrealismUI download url, default is (surrealism_ui 0.3.4)<br />
    /// https://github.com/Surrealism-All/SurrealismUI/releases/download/v0.3.4/surrealism-ui_0.3.4.zip
    url: &'static str,
    /// SurrealismUI doanload and decompression directory, default is<br />
    /// env::var_os("OUT_DIR").join("jui")
    pub out_dir: PathBuf,
    /// SurrealismUI alias name, default is surrealism
    pub alias: String,
}

/// custom method
impl SurrealismUI {
    /// new with url
    pub fn new_with_url(url: SurrealismUrl) -> Self {
        Self {
            url: url.get_url(),
            ..Default::default()
        }
    }

    /// new with alias
    pub fn new_with_alias(alias: impl Into<String>) -> Self {
        Self {
            alias: alias.into(),
            ..Default::default()
        }
    }

    /// new with url and alias
    pub fn new_with_url_alias(url: SurrealismUrl, alias: impl Into<String>) -> Self {
        Self {
            url: url.get_url(),
            alias: alias.into(),
            ..Default::default()
        }
    }

    /// new surrealism ui config
    pub fn new(url: SurrealismUrl, out_dir: PathBuf, alias: impl Into<String>) -> Self {
        Self {
            url: url.get_url(),
            out_dir,
            alias: alias.into(),
        }
    }

    /// set surrealism ui download url
    pub fn set_url(mut self, url: SurrealismUrl) -> Self {
        self.url = url.get_url();
        self
    }

    /// set out_dir
    pub fn set_out_dir(mut self, out_dir: PathBuf) -> Self {
        self.out_dir = out_dir;
        self
    }

    /// set surrealism ui alias
    pub fn set_alias(mut self, alias: impl Into<String>) -> Self {
        self.alias = alias.into();
        self
    }
}

/// support default
impl Default for SurrealismUI {
    fn default() -> Self {
        // build default out dir
        let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR is None");
        let out_dir = PathBuf::from(out_dir).join("jui");

        Self {
            url: SurrealismUrl::GITHUB.get_url(),
            out_dir,
            alias: "surrealism".into(),
        }
    }
}

/// custom method
impl SurrealismUI {
    /// build surrealism ui library path
    pub fn build_library_path(&self) -> HashMap<String, PathBuf> {
        let file = self.try_download().expect(&format!("download surrealism_ui[{:?}] to path[{:?}] fail", self.url, self.out_dir));
        let out_dir = self.try_extract(file).expect(&format!("extract surrealism_ui[{:?}] fail", self.out_dir));

        let mut index_slint = out_dir.join("index");
        index_slint.set_extension("slint");

        HashMap::from([
            (format!("{}_all", self.alias), index_slint),
            (format!("{}_icons", self.alias), out_dir.join("icons")),
            (self.alias.clone(), out_dir.join("src"))
        ])
    }

    /// try extract surrealism ui
    fn try_extract(&self, file: File) -> zip::result::ZipResult<PathBuf> {
        let out_dir = self.out_dir.join(surrealism_url::EXTRACT_NAME);
        if out_dir.exists() {
            return Ok(out_dir);
        }

        zip::ZipArchive::new(file)?.extract(&self.out_dir)?;
        Ok(out_dir)
    }

    /// try_download surrealism ui
    fn try_download(&self) -> anyhow::Result<File> {
        // rename download path
        let mut out_dir = self.out_dir.clone();
        if out_dir.is_file() {
            return Err(anyhow::anyhow!("out dir is file"));
        }
        out_dir = out_dir.join(surrealism_url::EXTRACT_NAME);
        out_dir.set_extension("zip");

        // if file is downloaded, open and return it
        if out_dir.exists() {
            return Ok(jui_file::just_open_file(&out_dir)?);
        }

        let bytes = reqwest::blocking::get(self.url)?.bytes()?;
        let mut file = jui_file::open_create_file(&out_dir)?;
        file.write_all(&bytes.to_vec())?;
        file.flush()?;

        Ok(file)
    }
}
