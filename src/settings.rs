use super::exit;
use console::style;
use itex_derive::itex_settings;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

const DEFAULT_TEX_FILENAME: &str = "main.tex";

#[itex_settings]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Settings {
    tex_filename: Option<String>,
    compile_bib: Option<bool>,
    debug: Option<bool>,
    output_dir: Option<PathBuf>,
    build_artifacts_folder: Option<String>,
    draft_mode: Option<bool>,
    clean: Option<bool>,
}

impl Settings {
    pub fn tex_filename(&self) -> String {
        self.tex_filename.clone().unwrap_or(DEFAULT_TEX_FILENAME.to_string())
    }

    pub fn compile_bib(&self) -> bool {
        self.compile_bib.unwrap_or(true)
    }

    pub fn debug(&self) -> bool {
        if std::env::var("ITEX_DEBUG").unwrap_or("FALSE".to_string()) == *"TRUE" {
            return true;
        }

        self.debug.unwrap_or(false)
    }

    pub fn output_dir(&self) -> PathBuf {
        self.output_dir.clone().unwrap_or(PathBuf::from("./out"))
    }

    pub fn build_artifacts_folder(&self) -> String {
        self.build_artifacts_folder.clone().unwrap_or("build_artifacts".to_string())
    }

    pub fn build_artifacts_path(&self) -> PathBuf {
        let mut output: PathBuf = self.output_dir();
        output.push(self.build_artifacts_folder());

        output
    }

    pub fn ensure_build_artifacts_path_exists(&self) {
        let build_artifacts_path = self.build_artifacts_path();
        if build_artifacts_path.is_dir() {
            return;
        }
        if !self.output_dir().is_dir() {
            std::fs::create_dir(self.output_dir()).unwrap();
        }

        std::fs::create_dir(build_artifacts_path).unwrap();
    }

    pub fn draft_mode(&self) -> bool {
        self.draft_mode.unwrap_or(false)
    }

    pub fn clean(&self) -> bool {
        self.clean.unwrap_or(false)
    }

    pub fn tex_filename_without_extension(&self) -> String {
        self.tex_filename().split('.').next().unwrap().to_string()
    }

    pub fn check_tex_filename_is_set(&self) {
        if self.tex_filename.is_none() {
            println!("{}", style("itex_filename is not set").red().bold());
            println!("{}", style("\titex set tex_filename <name of your .tex file>"));

            if !PathBuf::from(DEFAULT_TEX_FILENAME).is_file() {
                exit!(1);
            }
        }
    }

    pub fn find_and_parse_toml() -> Self {
        let mut path = std::env::current_dir().unwrap();
        path.push("itex-build.toml");

        let toml_file: PathBuf = if path.is_file() {
            path.to_owned()
        } else if path.with_file_name(".itex-build.toml").is_file() {
            path.to_owned().with_file_name(".itex-build.toml")
        } else {
            println!("{}", style("No itex build file found, please create one.").red().bold());
            exit!(0);
        };

        let build_file = std::fs::read_to_string(toml_file);
        if build_file.is_err() {
            println!("{}", style("Failed to read itex build file").red().bold());
            exit!(0);
        }
        let build_file = build_file.unwrap();

        let build_toml: Settings = toml::from_str(build_file.as_str()).unwrap();

        build_toml
    }
}
