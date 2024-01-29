use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use cbsk_base::anyhow;

/// surrealism ui config
pub struct SurrealismUI {
    /// SurrealismUI download url, default is (surrealism_ui 0.3.4)<br />
    /// https://github.com/Surrealism-All/SurrealismUI/releases/download/v0.3.4/surrealism-ui_0.3.4.zip
    pub url: String,
    /// SurrealismUI doanload and decompression directory, default is<br />
    /// env::var_os("OUT_DIR").join("jui")
    pub out_dir: PathBuf,
    /// SurrealismUI alias name, default is surrealism
    pub alias: String,
}

/// support default
impl Default for SurrealismUI {
    fn default() -> Self {
        // build default out dir
        let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR is None");
        let out_dir = PathBuf::from(out_dir).join("jui");

        Self {
            url: "https://github.com/Surrealism-All/SurrealismUI/releases/download/v0.3.4/surrealism-ui_0.3.4.zip".into(),
            out_dir,
            alias: "surrealism".into(),
        }
    }
}

/// custom method
impl SurrealismUI {
    /// build surrealism ui library path
    pub fn build_library_path(&self) -> HashMap<String, PathBuf> {
        let file = self.try_download().expect(&format!("download surrealism_ui[{}] to path[{:?}] fail", self.url, self.out_dir));
        let out_dir = self.try_extract(file).expect(&format!("extract surrealism_ui[{:?}] fail", self.out_dir));

        let mut index_slint = out_dir.join("index");
        index_slint.set_extension("slint");

        HashMap::from([
            (format!("{}_all", self.alias), index_slint),
            (self.alias.clone(), out_dir.join("src"))
        ])
    }

    /// try extract surrealism ui
    fn try_extract(&self, file: File) -> zip::result::ZipResult<PathBuf> {
        // TODO extract bugs present, currently, only 0.3.4 is supported
        let mut out_dir = self.out_dir.clone();
        if self.is_0_3_4() {
            out_dir = out_dir.join("surrealism-ui_0.3.4");
        }
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
        out_dir = out_dir.join("surrealism_ui");
        out_dir.set_extension("zip");

        // if file is downloaded, open and return it
        if out_dir.exists() {
            return Ok(jui_file::just_open_file(&out_dir)?);
        }

        let bytes = reqwest::blocking::get(self.url.as_str())?.bytes()?;
        let mut file = jui_file::open_create_file(&out_dir)?;
        file.write_all(&bytes.to_vec())?;
        file.flush()?;

        Ok(file)
    }

    /// check surrealism_ui version is 0.3.4
    fn is_0_3_4(&self) -> bool {
        self.url.eq("https://github.com/Surrealism-All/SurrealismUI/releases/download/v0.3.4/surrealism-ui_0.3.4.zip")
    }
}
