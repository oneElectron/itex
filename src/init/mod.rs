mod files;
mod template_info;
mod template_path;

use super::exit;
use console::style;
use std::{fs, path::PathBuf, string::String};
use template_path::find_templates_folder;

#[cfg(feature = "updater")]
use super::updater::download_templates;

const ITEX_BUILD_FILE: &str = r#"default_filename = "main"

"#;

pub fn copy_template(name: String, output_path: PathBuf, disable_os_search: bool) {
    create_build_file(output_path.clone());
    let path_to_templates = find_templates_folder(disable_os_search);
    let mut path_to_templates = match path_to_templates {
        Ok(p) => p,
        #[cfg(feature = "updater")]
        Err(1) => {
            download_templates(true);
            match find_templates_folder(disable_os_search) {
                Ok(p) => p,
                _ => {
                    exit!(0);
                }
            }
        }
        Err(_) => {
            exit!(0);
        }
    };

    path_to_templates.push(name);

    if cfg!(debug_assertions) {
        println!(
            "{} template path: {}",
            style("[DEBUG - copy_template]:").green(),
            path_to_templates.to_str().unwrap()
        );
    }
    if !path_to_templates.is_dir() {
        println!("{}", style("Could not find a template with the name provided").red().bold());
        println!("{}", style("Use itex list to get a list of available templates"));
        exit!(0);
    }

    let path_to_templates = PathBuf::from(path_to_templates.to_str().unwrap().trim());

    let mut pwd = output_path;

    pwd.push("file.txt");

    if cfg!(debug_assertions) {
        println!(
            "{} output dir = {}",
            style("[DEBUG - copy_template]:").green(),
            pwd.clone().to_str().unwrap()
        );
    }

    // dry run: find any files in the current folder that will conflict with the template files
    match files::copy_files(path_to_templates.clone(), pwd.clone(), true) {
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
    if files::copy_files(path_to_templates, pwd, false).is_err() {
        println!("Unexpected error")
    }
}

pub fn list_template_names(disable_os_search: bool) {
    println!("available template names:");
    let template_folder = find_templates_folder(disable_os_search);
    let template_folder = match template_folder {
        Ok(p) => p,
        #[cfg(feature = "updater")]
        Err(1) => {
            download_templates(true);
            exit!(0);
        }
        Err(_) => {
            exit!(0);
        }
    };

    for folder in fs::read_dir(template_folder).unwrap() {
        println!("    {}", folder.unwrap().file_name().to_str().unwrap());
    }
}

pub fn get_template_info(name: String, disable_os_search: bool) -> String {
    let path_to_templates = find_templates_folder(disable_os_search);
    let mut path_to_templates = match path_to_templates {
        Ok(p) => p,
        #[cfg(feature = "updater")]
        Err(1) => {
            download_templates(true);
            match find_templates_folder(disable_os_search) {
                Ok(p) => p,
                _ => {
                    exit!(0);
                }
            }
        }
        Err(_) => {
            exit!(0);
        }
    };

    path_to_templates.push(name);

    let info = template_info::get_template_info(path_to_templates);

    println!("{}: {}", info.name, info.description);

    info.description
}

pub fn create_build_file(path: PathBuf) {
    let mut path = path;
    path.push("itex-build.toml");
    if !path.is_file() {
        let output = std::fs::write(PathBuf::from("./itex-build.toml"), ITEX_BUILD_FILE);
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
            std::fs::remove_file(file.unwrap().path()).unwrap();
        }
    }

    #[test]
    fn default_config() {
        let mut out_dir = PathBuf::from("test_resources/test_cases/init/default_config/");
        assert!(!PathBuf::from("./test_resources/test_cases/init/default_config/main.tex").exists());

        super::copy_template("default".to_string(), out_dir.clone(), true);

        out_dir.push("itex-build.toml");
        assert!(out_dir.with_file_name("main.tex").is_file());
        assert!(!out_dir.with_file_name("itex-info.toml").is_file());
        cleanup_folder(out_dir.parent().unwrap().to_path_buf());
    }

    #[test]
    fn template_info() {
        let out = super::get_template_info("default".to_string(), true);

        assert_eq!(out, "The default template.".to_string());
    }

    #[test]
    fn list_templates() {
        super::list_template_names(true);
    }
}
