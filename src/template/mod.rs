mod files;
mod template_info;
mod template_path;

use super::template_updater::download_templates;
use std::{fs, path::PathBuf, process::exit, string::String};
use template_path::find_templates_folder;

pub fn copy_template(name: String, output_path: PathBuf, disable_os_search: bool) {
    let path_to_templates = find_templates_folder(disable_os_search);
    let mut path_to_templates = match path_to_templates {
        Ok(p) => p,
        Err(1) => {
            download_templates();
            match find_templates_folder(disable_os_search) {
                Ok(p) => p,
                _ => exit(0),
            }
        }
        Err(_) => exit(0),
    };

    path_to_templates.push(name);

    if cfg!(debug_assertions) {
        println!(
            "[DEBUG]: template path: {}",
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
    let mut pwd = output_path;

    pwd.push("file.txt");

    if cfg!(debug_assertions) {
        println!("[DEBUG]: output dir = {}", pwd.clone().to_str().unwrap());
    }

    // dry run: find any files in the current folder that will conflict with the template files
    match files::copy_files(path_to_templates.clone(), pwd.clone(), true) {
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
    if files::copy_files(path_to_templates, pwd, false).is_err() {
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

pub fn get_template_info(name: String, disable_os_search: bool) -> String {
    let path_to_templates = find_templates_folder(disable_os_search);
    let mut path_to_templates = match path_to_templates {
        Ok(p) => p,
        Err(1) => {
            download_templates();
            match find_templates_folder(disable_os_search) {
                Ok(p) => p,
                _ => exit(0),
            }
        }
        Err(_) => exit(0),
    };

    path_to_templates.push(name);

    let info = template_info::get_template_info(path_to_templates);

    println!("{}: {}", info.name, info.description);

    info.description
}

#[cfg(test)]
mod tests {
    fn setup_out_folder() -> std::path::PathBuf {
        let output_folder = std::path::PathBuf::from("./out");
        if output_folder.exists() && !output_folder.is_dir() {
            panic!("[TESTS]: Tests use the out folder as an output directory. Please remove the file called out");
        }
        if !output_folder.is_dir() {
            std::fs::create_dir(output_folder.clone()).expect("Could not create out folder");
        }

        output_folder
    }

    fn cleanup() {
        let clean_dir = setup_out_folder();

        std::fs::remove_dir_all(clean_dir.clone()).expect("Could not cleanup out dir");
        std::fs::create_dir(clean_dir).expect("could not create dir for testing");
    }

    #[test]
    fn default_config() {
        cleanup();
        let out_dir = setup_out_folder();
        let mut files = out_dir.clone();
        files.push("Makefile");

        super::copy_template("default".to_string(), out_dir, true);

        assert!(files.is_file());
        assert!(files.with_file_name("main.tex").is_file());
        assert!(!files.with_file_name("itex-info.json").is_file());

        cleanup();
    }

    #[test]
    fn template_info() {
        let out = super::get_template_info("default".to_string(), true);

        assert_eq!(
            out,
            "The default template. Contains just enough to get started.".to_string()
        );
    }

    #[test]
    fn list_templates() {
        super::list_template_names(false);
    }
}
