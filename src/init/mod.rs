mod files;

use crate::prelude::*;
use console::style;
use log::trace;
use std::{fs, path::PathBuf, string::String};

const ITEX_BUILD_FILE: &str = r#"default_filename = "main"
draft_mode = false
debug = false
output_dir = "./out"
compile_bib = true
"#;

pub fn init(name: String, output_path: PathBuf, search_path: Option<PathBuf>, disable_os_search: bool) {
    let path_to_templates = resolve_template(&name, disable_os_search, &search_path);

    trace!("template path = {}", path_to_templates.to_str().unwrap());

    if !path_to_templates.is_dir() {
        println!("{}", style("Could not find a template with the name provided").red().bold());
        println!("{}", style("Use itex list to get a list of available templates"));
        exit!(0);
    }

    let path_to_templates = PathBuf::from(path_to_templates.to_str().unwrap().trim());

    let mut pwd = output_path.clone();

    pwd.push("file.txt");

    trace!("output dir = {}", pwd.clone().to_str().unwrap());

    // dry run: find any files in the current folder that will conflict with the template files
    match files::copy_files(path_to_templates.clone(), &pwd, true) {
        Err(files::CopyFilesExitCode::SomeFilesExist) => {
            println!("Remove these files before running itex");
            exit!(0);
        }
        Err(files::CopyFilesExitCode::AllFilesExist) => {
            println!("All of the files in the template listed exist in this folder already");
            exit!(0);
        }
        _ => {}
    }

    // copy template to current directory
    if files::copy_files(path_to_templates, &pwd, false).is_err() {
        println!("Unexpected error")
    }

    create_build_file(output_path.clone());
}

pub fn list_template_names(search_path: Option<PathBuf>, disable_os_search: bool) {
    let template_folder = resolve_templates_folder(disable_os_search, &search_path);

    println!("available template names:");
    for folder in fs::read_dir(template_folder).unwrap() {
        let filename = folder.unwrap().file_name();
        let filename = filename.to_string_lossy();

        if filename.ends_with(".toml") {
            continue;
        }
        println!("    {}", filename);
    }
}

pub fn create_build_file(path: PathBuf) {
    let mut path = path;
    path.push("itex-build.toml");
    if !path.is_file() {
        let output = std::fs::write(path, ITEX_BUILD_FILE);
        if output.is_err() {
            println!("{}", style("Could not create itex-build.toml file").red().bold())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    fn cleanup_folder(path: PathBuf) {
        let dir = std::fs::read_dir(path).unwrap();
        for file in dir {
            if file.as_ref().unwrap().path().file_name().unwrap() == ".gitignore" {
                continue;
            }
            std::fs::remove_file(file.unwrap().path()).unwrap_or_else(|_| {});
        }
    }

    #[test]
    fn default_config() {
        let mut out_dir = PathBuf::from("test_resources/test_cases/init/default_config/");
        assert!(!PathBuf::from("./test_resources/test_cases/init/default_config/main.tex").exists());
        cleanup_folder(out_dir.parent().unwrap().to_path_buf());

        super::init("default".to_string(), out_dir.clone(), None, true);

        out_dir.push("itex-build.toml");
        assert!(out_dir.is_file());
        assert!(out_dir.with_file_name("main.tex").is_file());
        assert!(!out_dir.with_file_name("itex-info.toml").is_file());
        cleanup_folder(out_dir.parent().unwrap().to_path_buf());
    }

    #[test]
    fn list_templates() {
        super::list_template_names(None, true);
    }
}
