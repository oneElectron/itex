#![allow(dead_code)]
use super::exit;
use console::style;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::{Path, PathBuf};

const DEFAULT_DEFAULT_FILENAME: &str = "main";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Settings {
    default_filename: Option<String>,
    tex_filename: Option<String>,
    compile_bib: Option<bool>,
    debug: Option<bool>,
    output_dir: Option<PathBuf>,
}

impl Settings {
    pub fn tex_filename(&self) -> String {
        self.tex_filename
            .clone()
            .unwrap_or(self.default_filename.clone().unwrap_or("main".to_string()) + ".tex")
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

    pub fn compile_bib(&self, path: Option<PathBuf>) -> bool {
        if path.is_none() && self.compile_bib.is_none() {
            return false;
        } else if path.is_some() && self.compile_bib.is_none() {
            return contains_file_with_extension(path.unwrap(), "bib");
        }

        self.compile_bib.unwrap()
    }

    pub fn debug(&self) -> bool {
        if std::env::var("ITEX_DEBUG").unwrap_or("FALSE".to_string()) == *"TRUE" {
            return true;
        }

        self.debug.unwrap_or(false)
    }
}

impl Settings {
    pub fn set_default_filename(&mut self, filename: String) {
        self.default_filename = Some(filename);
    }

    pub fn set_tex_filename(&mut self, filename: String) {
        self.tex_filename = Some(filename);
    }

    pub fn set_compile_bib(&mut self, compile_bib: bool) {
        self.compile_bib = Some(compile_bib);
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = Some(debug);
    }
}

impl Settings {
    pub fn find_and_parse_toml(path: &Path) -> Self {
        let mut path = path.to_owned();
        path.push("itex-build.toml");

        let toml_file: PathBuf = if path.is_file() {
            path.to_owned()
        } else if path.with_file_name(".itex-build.toml").is_file() {
            path.to_owned().with_file_name(".itex-build.toml")
        } else {
            println!("{}", style("No itex build file found, please create one.").red().bold());
            exit!(0);
        };

        #[cfg(debug_assertions)]
        println!("Path inside find_and_parse_toml: {:?}", toml_file);

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

impl fmt::Display for Settings {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        self.print_default_filename();
        self.print_tex_filename();
        self.print_compile_bib();
        self.print_debug();

        fmt::Result::Ok(())
    }
}

impl Settings {
    pub fn print_default_filename(&self) -> Option<String> {
        match &self.default_filename {
            Some(value) => {
                println!(
                    "{} = {value}  (default: {DEFAULT_DEFAULT_FILENAME})",
                    style("default_filename").blue().bold()
                );
                Some(value.clone())
            }
            None => {
                println!(
                    "{} is not set (default: {DEFAULT_DEFAULT_FILENAME})",
                    style("default_filename").blue().bold()
                );
                None
            }
        }
    }

    pub fn print_tex_filename(&self) -> Option<String> {
        match &self.tex_filename {
            Some(value) => {
                println!("{} = {value}  (default: No default)", style("tex_filename").blue().bold());
                Some(value.clone())
            }
            None => {
                println!(
                    "{} is not set, value is inherited from default_filename: {}.tex",
                    style("tex_filename").blue().bold(),
                    self.default_filename.clone().unwrap_or("main".to_string())
                );
                None
            }
        }
    }

    pub fn print_compile_bib(&self) -> Option<bool> {
        match self.compile_bib {
            Some(value) => {
                println!("{} = {value}  (default: false)", style("compile_bib").blue().bold());
                Some(value)
            }
            None => {
                println!("{} is not set (default: false)", style("compile_bib").blue().bold());
                None
            }
        }
    }

    pub fn print_debug(&self) -> Option<bool> {
        match self.debug {
            Some(value) => {
                println!("{} = {value}  (default: false)", style("debug").blue().bold());
                Some(value)
            }
            None => {
                println!("{} is not set (default: false)", style("debug").blue().bold());
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_get() {
        let output = crate::get(
            Some("default_filename".to_string()),
            PathBuf::from("./test_resources/test_cases/settings/get/"),
        )
        .unwrap();

        assert_eq!(output.unwrap(), "main");
    }

    #[test]
    fn settings_set() {
        let path = PathBuf::from("test_resources/test_cases/settings/set");
        assert!(path.is_dir());

        crate::set(Some("default_filename".to_string()), Some("Hello".to_string()), path.clone());

        let build = crate::get(Some("default_filename".to_string()), path.clone());

        assert_eq!(build.unwrap().unwrap(), "Hello".to_string());

        crate::set(
            Some("default_filename".to_string()), // Set it back just in case
            Some("main".to_string()),
            path.clone(),
        );

        let build = crate::get(Some("default_filename".to_string()), path);

        assert_eq!(build.unwrap().unwrap(), "main".to_string());
    }

    #[test]
    fn settings_set_tex_filename() {
        let path = PathBuf::from("test_resources/test_cases/settings/set_tex_filename");
        assert!(path.is_dir());

        crate::set(Some("tex_filename".to_string()), Some("Hello".to_string()), path.clone());

        let build = crate::get(Some("tex_filename".to_string()), path.clone());

        assert_eq!(build.unwrap().unwrap(), "Hello".to_string());

        crate::set(
            Some("tex_filename".to_string()), // Set it back just in case
            Some("main".to_string()),
            path.clone(),
        );

        let build = crate::get(Some("tex_filename".to_string()), path);

        assert_eq!(build.unwrap().unwrap(), "main".to_string());
    }

    #[test]
    fn settings_set_with_dotfile() {
        let path = PathBuf::from("test_resources/test_cases/settings/set_with_dotfile");
        assert!(path.is_dir());
        crate::set(Some("default_filename".to_string()), Some("Hello".to_string()), path.clone());

        let build = crate::get(Some("default_filename".to_string()), path.clone());

        assert_eq!(build.unwrap().unwrap(), "Hello".to_string());

        crate::set(
            Some("default_filename".to_string()), // Set it back just in case
            Some("main".to_string()),
            path.clone(),
        );

        let build = crate::get(Some("default_filename".to_string()), path);

        assert_eq!(build.unwrap().unwrap(), "main".to_string());
    }

    #[test]
    fn settings_folder_contains_extension() {
        let path = PathBuf::from("test_resources/test_cases/settings/folder_contains_extension");

        let output = contains_file_with_extension(path, "bib");

        assert!(output == true);
    }

    #[test]
    fn settings_folder_doesnt_contain_extension() {
        let path = PathBuf::from("test_resources/test_cases/settings/folder_doesnt_contain_extension");

        let output = contains_file_with_extension(path, "bib");

        assert!(output == false);
    }

    #[test]
    #[should_panic]
    fn settings_set_invalid_setting() {
        crate::set(
            Some("Hello, This is a bad setting".to_string()),
            Some("main".to_string()),
            PathBuf::from("test_resources/test_cases/settings/set_invalid_setting"),
        );
    }

    #[test]
    #[should_panic]
    fn settings_set_without_value() {
        crate::set(
            Some("default_filename".to_string()),
            None,
            PathBuf::from("test_resources/test_cases/settings_set_without_value"),
        );
    }

    #[test]
    #[should_panic]
    fn test_set_without_setting() {
        crate::set(None, None, PathBuf::from("./out"))
    }
}
