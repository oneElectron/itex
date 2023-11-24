use crate::prelude::*;
use std::path::PathBuf;

pub struct TemplateList {
    pub names: Vec<String>,
}

impl std::fmt::Display for TemplateList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", console::style("Templates available:").green().bright()).unwrap();
        for name in &self.names {
            writeln!(f, "  {}", name).unwrap();
        }

        std::fmt::Result::Ok(())
    }
}

impl TemplateList {
    pub fn from_path(search_path: Option<PathBuf>, disable_os_search: bool) -> Self {
        let template_folder = resolve_templates_folder(disable_os_search, &search_path);

        let mut template_names: Vec<String> = vec![];
        for folder in std::fs::read_dir(template_folder).unwrap() {
            let filename = folder.unwrap().file_name();
            let filename = filename.to_string_lossy();

            if filename.ends_with(".toml") {
                continue;
            }

            template_names.push(filename.to_string());
        }
        TemplateList { names: template_names }
    }
}

pub fn list(search_path: Option<PathBuf>, disable_os_search: bool) {
    let template_list = TemplateList::from_path(search_path, disable_os_search);

    print!("{}", template_list);
}

#[cfg(test)]
mod tests {
    #[test]
    fn list_templates() {
        super::TemplateList::from_path(None, true);
    }
}
