use crate::prelude::*;
use console::style;
use std::path::PathBuf;

pub fn clean(project_path: PathBuf, settings: &Settings) {
    let mut out_folder_path = project_path;
    out_folder_path.push(settings.output_dir());
    if !out_folder_path.is_dir() {
        println!("{}", style("could not find out dir").red().bold());
        exit!(1);
    }

    let out_folder_path = out_folder_path.read_dir().expect("Could not read dir");

    for file in out_folder_path {
        let path = file.unwrap();
        let filename = path.file_name();
        let filename = filename.to_str().unwrap();
        let path = path.path();

        if !include_file(filename) && path.is_file() && std::fs::remove_file(path).is_err() {
            println!("{}", style("failed to remove file in out folder").red().bold());
        }
    }

    clean_build_artifacts_folder(settings);
}

pub fn clean_build_artifacts_folder(settings: &Settings) {
    let build_artifacts_path = settings.build_artifacts_path();
    if build_artifacts_path.is_dir() {
        std::fs::remove_dir_all(settings.build_artifacts_path()).unwrap();
    }
}

pub fn include_file(filename: &str) -> bool {
    let binding = PathBuf::from(filename);
    let extension = binding.extension();
    if extension.is_none() {
        return true;
    }

    let extension = extension.unwrap().to_string_lossy();

    if extension == "pdf" {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ignore_file() {
        assert_eq!(include_file("main.pdf"), true);
        assert_eq!(include_file("main.log"), false);
    }
}
