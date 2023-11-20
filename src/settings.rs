#![allow(dead_code)]
use crate::prelude::*;
use console::style;
use itex_derive::itex_settings;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

const DEFAULT_DEFAULT_FILENAME: &str = "main";

#[itex_settings]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Settings {
    default_filename: Option<String>,
    tex_filename: Option<String>,
    compile_bib: Option<bool>,
    debug: Option<bool>,
    output_dir: Option<PathBuf>,
    draft_mode: Option<bool>,
    clean: Option<bool>,
}

impl Settings {
    pub fn default_filename(&self) -> String {
        self.default_filename.clone().unwrap_or(DEFAULT_DEFAULT_FILENAME.to_owned())
    }

    pub fn tex_filename(&self) -> String {
        self.tex_filename
            .clone()
            .unwrap_or(self.default_filename.clone().unwrap_or("main".to_string()) + ".tex")
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

    pub fn draft_mode(&self) -> bool {
        self.draft_mode.unwrap_or(false)
    }

    pub fn clean(&self) -> bool {
        self.clean.unwrap_or(true)
    }

    pub fn tex_filename_without_extension(&self) -> String {
        self.tex_filename
            .clone()
            .unwrap_or(self.default_filename.clone().unwrap_or("main".to_string()) + ".tex")
            .split('.')
            .next()
            .unwrap()
            .to_string()
    }

    pub fn check_tex_file_exists(&self) {
        if !PathBuf::from(self.tex_filename()).is_file() {
            println!(
                "{}{}",
                style(self.tex_filename()).red().bold(),
                style(" not found, you must either create it, or change the tex_filename option in your itex-build.toml")
                    .red()
                    .bold()
            );

            exit!(1);
        }
    }
}

impl Settings {
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

fn contains_file_with_extension(path: PathBuf, extension: &str) -> bool {
    let contents_of_dir = std::fs::read_dir(path).unwrap();
    for file in contents_of_dir {
        let file = file.unwrap().path();
        let file = file.extension();
        if file.is_none() {
            continue;
        }

        if file.unwrap().to_str().unwrap() == extension {
            return true;
        }
    }

    false
}
