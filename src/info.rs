use crate::prelude::*;
use console::style;
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
pub struct TemplateInfo {
    pub name: String,
    pub description: Option<String>,
    pub id: Option<i64>,
    pub website: Option<String>,
    pub author: Option<String>,
    pub excluded_files: Option<Vec<String>>,
}

impl std::fmt::Display for TemplateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", console::style(&self.name).blue().bright()).unwrap();
        if self.description.is_some() {
            write!(f, ": {}", self.description.as_ref().unwrap()).unwrap();
        }
        if self.author.is_some() {
            write!(f, "\n\tauthor: {}", self.author.as_ref().unwrap()).unwrap();
        }
        if self.website.is_some() {
            write!(f, "\n\thomepage: {}", self.website.as_ref().unwrap()).unwrap();
        }

        std::fmt::Result::Ok(())
    }
}

impl TemplateInfo {
    pub fn from_path(path: &Path) -> Self {
        let path = if path.file_name().unwrap().to_string_lossy() == "itex-info.toml" {
            path.to_owned()
        } else {
            path.to_owned().join("itex-info.toml")
        };

        let toml_str = std::fs::read_to_string(path);
        if toml_str.is_err() {
            println!("{}", style("Could not find info for template").red().bold());
            exit!(0);
        }
        let toml_str = toml_str.unwrap();

        let toml_str: Result<TemplateInfo, toml::de::Error> = toml::from_str(toml_str.as_str());

        unwrap_result!(toml_str, "Failed to parse toml")
    }
}

pub fn template_info(name: String, search_path: Option<PathBuf>, disable_os_search: bool) -> crate::info::TemplateInfo {
    let path_to_templates = resolve_template(&name, disable_os_search, &search_path);

    TemplateInfo::from_path(&path_to_templates)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn toml_file() {
        let output = TemplateInfo::from_path(&PathBuf::from("test_resources/default"));

        assert_eq!(output.name, "Default".to_string());
        assert_eq!(output.description.unwrap(), "The default template.".to_string());
        assert_eq!(output.id.unwrap(), 0);
        assert_eq!(output.website, None);
        assert_eq!(output.author, None);
    }
}
