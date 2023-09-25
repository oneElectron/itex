use crate::prelude::*;
use console::style;
use std::path::PathBuf;

pub fn get(setting: Option<String>, path: PathBuf) -> std::result::Result<Option<String>, u32> {
    let itex_build_toml = Settings::find_and_parse_toml(&path);

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
