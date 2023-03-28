use console::style;
use std::path::PathBuf;
use std::process::{exit, Command};

pub fn build(debug: bool) {
    let args = vec!["-output-directory", "./out/", "main.tex"];

    let output = Command::new("pdflatex").args(args).output();

    if output.is_err() {
        println!("{}", style("Error building pdf").red().bold());
    }

    if !debug {
        remove_files();
    }
}

fn remove_files() {
    let out_folder_path = PathBuf::from("./out");
    if !out_folder_path.is_dir() {
        println!("{}", style("could not find out dir").red().bold());
        exit(1);
    }

    let out_folder_path = out_folder_path.read_dir().expect("Could not read dir");

    for file in out_folder_path {
        let path = file.unwrap();
        let filename = path.file_name();
        let filename = filename.to_str().unwrap();
        let path = path.path();

        if !ignore_file(&filename) && path.is_file() {
            if std::fs::remove_file(path).is_err() {
                println!("{}", style("failed to remove file in out folder").red().bold());
            }
        }
    }
}

fn ignore_file(filename: &str) -> bool {
    let binding = PathBuf::from(filename.clone());
    let extension = binding.extension().unwrap().to_str().unwrap();

    if extension == "pdf" {
        return true;
    }
    return false;
}
