use super::exit;
use console::style;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::option::Option;
use std::path::PathBuf;

const DEFAULT_DEFAULT_FILENAME: &str = "main";

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub default_filename: Option<String>,
    pub tex_filename: Option<String>,
}

impl fmt::Display for Settings {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        self.print_default_filename();
        self.print_tex_filename();

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
}

pub fn find_and_parse_toml(path: PathBuf) -> Settings {
    let mut path = path;
    path.push("itex-build.toml");

    let toml_file: PathBuf = if path.is_file() {
        path.clone()
    } else if path.with_file_name(".itex-build.toml").is_file() {
        path.clone().with_file_name(".itex-build.toml")
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

pub fn set(setting: Option<String>, value: Option<String>, path: PathBuf) {
    if setting.is_none() {
        println!("{}", style("No value given for setting").red().bold());
        exit!(0);
    }
    if value.is_none() {
        println!("{}", style("No value given for setting").red().bold());
        exit!(0);
    }

    let setting = setting.unwrap();
    let setting = setting.as_str();
    let value = value.unwrap();

    let mut build_settings = find_and_parse_toml(path.clone());

    match setting {
        "default_filename" => build_settings.default_filename = Some(value),
        "tex_filename" => build_settings.tex_filename = Some(value),
        _ => {
            println!("{}", style("Invalid setting name").red().bold());
            exit!(0);
        }
    }

    let build_settings_str: Result<String, toml::ser::Error> = toml::to_string_pretty(&build_settings);
    let build_settings_str: String = build_settings_str.unwrap();

    let mut path = path;
    path.push("itex-build.toml");

    let mut path_with_dot = path.clone();
    path_with_dot.push("/.itex-build.toml");

    if path_with_dot.is_file() {
        if std::fs::write(path_with_dot, build_settings_str).is_err() {
            println!("{}", style("Failed to write to .itex-build.toml").red().bold());
        }
    } else if std::fs::write(path, build_settings_str).is_err() {
        println!("{}", style("Failed to write to .itex-build.toml").red().bold());
    }
}

pub fn get(setting: Option<String>, path: PathBuf) -> std::result::Result<Option<String>, u32> {
    let itex_build_toml = find_and_parse_toml(path);

    if setting.is_none() {
        println!("{}", itex_build_toml);
        return Ok(None);
    }
    let setting = setting.unwrap();
    let setting = setting.as_str();

    let output = match setting {
        "default_filename" => itex_build_toml.print_default_filename(),
        "tex_filename" => itex_build_toml.print_tex_filename(),
        _ => {
            println!("{}", style("Invalid setting name").red().bold());
            exit!(0);
        }
    };

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn settings_get() {
        let output = get(
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

        set(Some("default_filename".to_string()), Some("Hello".to_string()), path.clone());

        let build = get(Some("default_filename".to_string()), path.clone());

        assert_eq!(build.unwrap().unwrap(), "Hello".to_string());

        set(
            Some("default_filename".to_string()), // Set it back just in case
            Some("main".to_string()),
            path.clone(),
        );

        let build = get(Some("default_filename".to_string()), path);

        assert_eq!(build.unwrap().unwrap(), "main".to_string());
    }
    #[test]
    fn settings_set_tex_filename() {
        let path = PathBuf::from("test_resources/test_cases/settings/set_tex_filename");
        assert!(path.is_dir());

        set(Some("tex_filename".to_string()), Some("Hello".to_string()), path.clone());

        let build = get(Some("tex_filename".to_string()), path.clone());

        assert_eq!(build.unwrap().unwrap(), "Hello".to_string());

        set(
            Some("tex_filename".to_string()), // Set it back just in case
            Some("main".to_string()),
            path.clone(),
        );

        let build = get(Some("tex_filename".to_string()), path);

        assert_eq!(build.unwrap().unwrap(), "main".to_string());
    }
    #[test]
    fn settings_set_with_dotfile() {
        let path = PathBuf::from("test_resources/test_cases/settings/set_with_dotfile");
        assert!(path.is_dir());
        set(Some("default_filename".to_string()), Some("Hello".to_string()), path.clone());

        let build = get(Some("default_filename".to_string()), path.clone());

        assert_eq!(build.unwrap().unwrap(), "Hello".to_string());

        set(
            Some("default_filename".to_string()), // Set it back just in case
            Some("main".to_string()),
            path.clone(),
        );

        let build = get(Some("default_filename".to_string()), path);

        assert_eq!(build.unwrap().unwrap(), "main".to_string());
    }
    #[test]
    #[should_panic]
    fn settings_set_invalid_setting() {
        let output = set(
            Some("Hello, This is a bad setting".to_string()),
            Some("main".to_string()),
            PathBuf::from("test_resources/test_cases/settings/set_invalid_setting"),
        );
    }
    #[test]
    #[should_panic]
    fn settings_set_without_value() {
        set(
            Some("default_filename".to_string()),
            None,
            PathBuf::from("test_resources/test_cases/settings_set_without_value"),
        );
    }
    #[test]
    #[should_panic]
    fn test_set_without_setting() {
        set(None, None, PathBuf::from("./out"))
    }
}
