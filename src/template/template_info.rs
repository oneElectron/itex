use serde_derive::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct TemplateInfo {
    name: String,
    description: String,
    id: i64,
}

pub fn get_template_info(template_path: PathBuf) -> TemplateInfo {
    let mut path = template_path.clone();
    path.push("itex-info.json");

    let json_str =
        std::fs::read_to_string(path)
            .expect("could not find itex-info.json for template");

    parse_json(json_str.as_str())
}

fn parse_json(info: &str) -> TemplateInfo {
    let data: TemplateInfo = serde_json::from_str(info).unwrap();

    data
}

#[test]
fn json_parsing() {
    let data = r#"
        {
            "name": "Test Template",
            "description": "A very good template",
            "id": 43
        }"#;

    let output = super::template_info::parse_json(data);

    assert_eq!(output.name, "Test Template".to_string());
    assert_eq!(output.description, "A very good template".to_string());
    assert_eq!(output.id, 43);
}

#[test]
fn json_file() {
    let output = get_template_info(PathBuf::from("./test_resources/default"));

    assert_eq!(output.name, "Default".to_string());
    assert_eq!(output.description, "The default template. Contains just enough to get started.".to_string());
    assert_eq!(output.id, 0);
}
