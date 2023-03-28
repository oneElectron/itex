use serde_derive::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct TemplateInfo {
    pub name: String,
    pub description: String,
    pub id: i64,
}

pub fn get_template_info(template_path: PathBuf) -> TemplateInfo {
    let mut path = template_path;
    path.push("itex-info.json");

    let json_str = std::fs::read_to_string(path).expect("could not find itex-info.json for template");

    let data: TemplateInfo = serde_json::from_str(json_str.as_str()).unwrap();

    data
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn json_file() {
        let output = get_template_info(PathBuf::from("./test_resources/default"));

        assert_eq!(output.name, "Default".to_string());
        assert_eq!(
            output.description,
            "The default template.".to_string()
        );
        assert_eq!(output.id, 0);
    }
}
