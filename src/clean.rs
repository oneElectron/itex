use crate::prelude::*;
use console::style;
use std::path::PathBuf;

pub fn clean(project_path: PathBuf) {
    let mut out_folder_path = project_path;
    out_folder_path.push("out");
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
}

pub fn include_file(filename: &str) -> bool {
    let binding = PathBuf::from(filename);
    let extension = binding.extension().unwrap().to_str().unwrap();

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
