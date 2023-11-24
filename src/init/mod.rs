mod files;

use crate::prelude::*;
use console::style;
use std::path::PathBuf;

const ITEX_BUILD_FILE: &str = r#"tex_filename = "main.tex"
draft_mode = false
debug = false
output_dir = "./out"
"#;

pub fn init(name: String, output_path: PathBuf, search_path: Option<PathBuf>, disable_os_search: bool) {
    let path_to_templates = resolve_template(&name, disable_os_search, &search_path);

    log::trace!("template path = {}", path_to_templates.to_str().unwrap());

    if !path_to_templates.is_dir() {
        println!("{}", style("Could not find a template with the name provided").red().bold());
        println!("{}", style("Use itex list to get a list of available templates"));
        exit!(0);
    }

    let path_to_templates = PathBuf::from(path_to_templates.to_str().unwrap().trim());

    let template_info = TemplateInfo::from_path(&path_to_templates);

    let mut pwd = output_path.clone();

    pwd.push("file.txt");

    log::trace!("output dir = {}", pwd.clone().to_str().unwrap());

    // dry run: find any files in the current folder that will conflict with the template files
    match files::copy_files(path_to_templates.clone(), true, &template_info) {
        Err(files::CopyFilesExitCode::SomeFilesExist) => {
            println!("Remove these files before running itex");
            exit!(0);
        }
        Err(files::CopyFilesExitCode::AllFilesExist) => {
            println!("All files exist, have you run itex init before?");
            exit!(0);
        }
        _ => {}
    }

    // copy template to current directory
    if files::copy_files(path_to_templates, false, &template_info).is_err() {
        println!("Unexpected error");
    }

    create_build_file(output_path.clone());
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
