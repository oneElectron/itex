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
                println!("default_filename = {value}  (default: {DEFAULT_DEFAULT_FILENAME})");
                Some(value.clone())
            }
            None => {
                println!("default_filename is not set (default: {DEFAULT_DEFAULT_FILENAME})");
                None
            }
        }
    }

    pub fn print_tex_filename(&self) -> Option<String> {
        match &self.tex_filename {
            Some(value) => {
                println!("tex_filename = {value}  (default: No default)");
                Some(value.clone())
            }
            None => {
                println!(
                    "tex_filename is not set, value is inherited from default_filename: {}.tex",
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

    println!("{:?}", path.clone());

    let mut build_settings = find_and_parse_toml(path.clone());

    match setting {
        "default_filename" => build_settings.default_filename = Some(value),
        "tex_filename" => build_settings.tex_filename = Some(value),
        _ => println!("{}", style("Invalid setting name").red().bold()),
    }

    let build_settings_str: Result<String, toml::ser::Error> = toml::to_string_pretty(&build_settings);
    let build_settings_str: String = build_settings_str.unwrap();

    println!("{}", build_settings_str);

    let mut path = path.clone();
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
    use std::fs;

    const ITEX_BUILD_FILE: &str = r#"default_filename = "main"
tex_filename = "tex"

"#;

    fn setup_out_folder() {
        std::thread::sleep(std::time::Duration::from_millis(100)); // Leave this
        if !PathBuf::from("./out").exists() {
            fs::create_dir("./out").unwrap();
        }
        fs::write(PathBuf::from("./out/itex-build.toml"), ITEX_BUILD_FILE).unwrap();

        if PathBuf::from("./out/.itex-build.toml").is_file() {
            fs::remove_file(PathBuf::from("./out/.itex-build.toml")).unwrap();
        }
    }

    fn setup_out_folder_with_dotfile() {
        std::thread::sleep(std::time::Duration::from_millis(1000)); // Leave this
        if PathBuf::from("out").is_dir() {
            fs::remove_dir_all("out").expect("could not remove out folder");
        }
        fs::create_dir("out").expect("failed to create out folder");
        fs::write("out/.itex-build.toml", ITEX_BUILD_FILE).expect("Could not write to dotfile");

        if PathBuf::from("./out/itex-build.toml").is_file() {
            fs::remove_file(PathBuf::from("./out/itex-build.toml")).unwrap();
        }
    }

    #[test]
    fn test_get() {
        std::thread::sleep(std::time::Duration::from_millis(10));
        setup_out_folder();
        let output = get(Some("default_filename".to_string()), PathBuf::from("./out")).unwrap();

        assert_eq!(output.unwrap(), "main");
    }

    #[test]
    fn test_set() {
        setup_out_folder();
        set(
            Some("default_filename".to_string()),
            Some("Hello".to_string()),
            PathBuf::from("./out"),
        );

        let build = get(Some("default_filename".to_string()), PathBuf::from("./out"));

        assert_eq!(build.unwrap().unwrap(), "Hello".to_string());

        set(
            Some("default_filename".to_string()), // Set it back just in case
            Some("main".to_string()),
            PathBuf::from("./out"),
        );

        let build = get(Some("default_filename".to_string()), PathBuf::from("./out"));

        assert_eq!(build.unwrap().unwrap(), "main".to_string());
    }

    #[test]
    fn test_set_with_dotfile() {
        setup_out_folder_with_dotfile();

        assert!(PathBuf::from("out/.itex-build.toml").is_file());
        set(
            Some("default_filename".to_string()),
            Some("Hello".to_string()),
            PathBuf::from("./out"),
        );

        let build = get(Some("default_filename".to_string()), PathBuf::from("./out"));

        assert_eq!(build.unwrap().unwrap(), "Hello".to_string());

        set(
            Some("default_filename".to_string()), // Set it back just in case
            Some("main".to_string()),
            PathBuf::from("./out"),
        );

        let build = get(Some("default_filename".to_string()), PathBuf::from("./out"));

        assert_eq!(build.unwrap().unwrap(), "main".to_string());
    }

    #[test]
    #[should_panic]
    fn test_set_without_value() {
        set(Some("default_filename".to_string()), None, PathBuf::from("./out"))
    }

    #[test]
    #[should_panic]
    fn test_set_without_setting() {
        set(None, None, PathBuf::from("./out"))
    }
}
