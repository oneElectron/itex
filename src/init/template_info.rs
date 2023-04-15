use super::exit;
use console::style;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct TemplateInfo {
    pub name: String,
    pub description: String,
    pub id: i64,
}

pub fn get_template_info(template_path: PathBuf) -> TemplateInfo {
    let mut path = template_path;
    path.push("itex-info.toml");

    #[cfg(test)]
    println!("path to itex-info.toml: {:?}", path.clone());

    let toml_str = std::fs::read_to_string(path);
    if toml_str.is_err() {
        println!("{}", style("Could not find info for template").red().bold());
        exit!(0);
    }
    let toml_str = toml_str.unwrap();

    let data: TemplateInfo = toml::from_str(toml_str.as_str()).unwrap();

    data
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn toml_file() {
        let output = get_template_info(PathBuf::from("test_resources/default"));

        assert_eq!(output.name, "Default".to_string());
        assert_eq!(output.description, "The default template.".to_string());
        assert_eq!(output.id, 0);
    }
}
