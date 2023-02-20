mod tests;

use serde_derive::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct TemplateInfo {
    name: String,
    description: String,
    id: i64,
}

pub fn get_name_description(template_path: PathBuf) -> TemplateInfo {
    let mut path = template_path.clone();
    path.push("itex-info.json");

    let json_str =
        std::fs::read_to_string(path).expect("could not find itex-info.json for template");

    parse_json(json_str.as_str())
}

fn parse_json(info: &str) -> TemplateInfo {
    let data: TemplateInfo = serde_json::from_str(info).unwrap();

    data
}
