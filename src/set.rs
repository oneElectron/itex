use crate::prelude::*;
use console::style;

pub fn set(setting: Option<String>, value: Option<String>) {
    if setting.is_none() {
        println!("{}", style("No value given for setting").red().bold());
        exit!(0);
    }
    if value.is_none() {
        println!("{}", style("No value given for setting").red().bold());
        exit!(0);
    }

    let setting = setting.unwrap();
    let value = value.unwrap();

    let mut build_settings = Settings::find_and_parse_toml();

    match setting.as_str() {
        "default_filename" => build_settings.set_default_filename(value),
        "tex_filename" => build_settings.set_tex_filename(value),
        "compile_bib" => build_settings.set_compile_bib(match value.as_str() {
            "true" => true,
            "false" => false,
            _ => {
                println!("Invalid value");
                exit!(0);
            }
        }),
        "debug" => build_settings.set_debug(match value.as_str() {
            "true" => true,
            "false" => false,
            _ => {
                println!("Invalid value");
                exit!(0);
            }
        }),
        _ => {
            println!("{}", style("Invalid setting name").red().bold());
            exit!(0);
        }
    }

    let build_settings_str: Result<String, toml::ser::Error> = toml::to_string_pretty(&build_settings);
    let build_settings_str: String = build_settings_str.unwrap();

    let mut path = std::env::current_dir().unwrap();
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
