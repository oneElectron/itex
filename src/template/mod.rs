mod files;
mod template_info;
mod template_path;

use super::runtime_helper::Options;
use super::template_updater::download_templates;
use std::{env, fs, path::PathBuf, process::exit, string::String};
use template_path::find_templates_folder;

pub fn copy_template(name: String, runtime_options: Options) {
    let path_to_templates = find_templates_folder(runtime_options.disable_os_search);
    let mut path_to_templates = match path_to_templates {
        Ok(p) => p,
        Err(1) => {
            download_templates();
            match find_templates_folder(runtime_options.disable_os_search) {
                Ok(p) => p,
                _ => exit(0),
            }
        }
        Err(_) => exit(0),
    };

    path_to_templates.push(name);

    if cfg!(debug_assertions) {
        println!(
            "[DEBUG] template path: {}",
            path_to_templates.to_str().unwrap()
        );
    }
    if !path_to_templates.is_dir() {
        println!("could not find a template with the name provided");
        println!("use itex --list to get a list of available templates");
        exit(1);
    }

    let path_to_templates = PathBuf::from(path_to_templates.to_str().unwrap().trim());

    // find current dir
    let mut pwd = env::current_dir().expect("could not find current directory");

    pwd.push("file.txt");

    // dry run: find any files in the current folder that will conflict with the template files
    match files::copy_files(path_to_templates.clone(), true) {
        Err(files::CopyFilesExitCode::SomeFilesExist) => {
            println!("Remove these files before running itex");
            exit(0)
        }
        Err(files::CopyFilesExitCode::AllFilesExist) => {
            println!("All of the files in the template listed exist in this folder already");
            exit(0)
        }
        _ => {}
    }

    // copy template to current directory
    if files::copy_files(path_to_templates, false).is_err() {
        println!("Unexpected error")
    }
}

pub fn list_template_names(disable_os_search: bool) {
    println!("available template names:");
    let template_folder = find_templates_folder(disable_os_search);
    let template_folder = match template_folder {
        Ok(p) => p,
        Err(1) => {
            download_templates();
            exit(0);
        }
        Err(_) => exit(0),
    };

    for folder in fs::read_dir(template_folder).unwrap() {
        println!("    {}", folder.unwrap().file_name().to_str().unwrap());
    }
}

pub fn get_template_info(name: String, runtime_options: Options) {
    let path_to_templates = find_templates_folder(runtime_options.disable_os_search);
    let mut path_to_templates = match path_to_templates {
        Ok(p) => p,
        Err(1) => {
            download_templates();
            match find_templates_folder(runtime_options.disable_os_search) {
                Ok(p) => p,
                _ => exit(0),
            }
        }
        Err(_) => exit(0),
    };

    path_to_templates.push(name);

    let info = template_info::get_template_info(path_to_templates);

    println!("{}: {}", info.name, info.description);    
}