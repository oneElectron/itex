#![allow(dead_code)]
use crate::prelude::*;

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
        self.build_artifacts_folder.clone().unwrap_or("itex-build".to_string())
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

    #[allow(unused)]
    pub fn from_local() -> Self {
        Self::check_not_in_config_folder();
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

    pub fn from_global() -> Self {
        Self::check_not_in_config_folder();

        let mut global_build_toml: Option<Self> = None;

        if Self::global_settings_path().is_file() {
            let path = Self::global_settings_path();
            let global_build_toml_str = std::fs::read_to_string(path).unwrap();
            let global_build_toml_err: Result<Settings, toml::de::Error> = toml::from_str(&global_build_toml_str);
            if let Ok(global_build_toml_err) = global_build_toml_err {
                global_build_toml = Some(global_build_toml_err);
            }
        }
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

        Self::merge_global_and_local(global_build_toml, build_toml)
    }

    fn check_not_in_config_folder() {
        if Self::global_settings_path().parent().unwrap() == std::env::current_dir().unwrap() {
            println!(
                "{}",
                style("Current dir and global config dir are the same.\nPlease do not build in itex config folder")
                    .red()
                    .bold()
            );
            exit!(0);
        }
    }

    fn global_settings_path() -> PathBuf {
        #[cfg(unix)]
        {
            let home = std::env::var("HOME").unwrap();
            PathBuf::from(home).join("/.config/itex/itex-build.toml")
        }

        #[cfg(windows)]
        {
            let home = std::env::var("HOME").unwrap();
            PathBuf::from(home).join("/AppData/Local/ITex/itex-build.toml")
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_settings_merge() {
        let mut local = super::Settings::empty();
        local.build_artifacts_folder = Some("local_build".to_string());
        local.compile_bib = Some(false);
        let mut global = super::Settings::empty();
        global.build_artifacts_folder = Some("global_build".to_string());
        global.clean = Some(true);

        let output = super::Settings::merge_global_and_local(Some(global), local);

        assert_eq!(output.build_artifacts_folder, Some("local_build".to_string()));
        assert_eq!(output.compile_bib, Some(false));
        assert_eq!(output.clean, Some(true));
    }
}
