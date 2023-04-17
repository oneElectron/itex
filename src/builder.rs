use super::exit;
use super::settings;
use console::style;
use std::path::PathBuf;
use std::process::Command;
use std::str;

pub fn build(debug: bool, project_path: PathBuf) {
    let build_settings = settings::find_and_parse_toml(project_path);
    let tex_filename = build_settings.tex_filename();
    let args = vec!["-output-directory", "./out/", tex_filename.as_str()];



    let output = Command::new("pdflatex").args(args.clone()).output();

    if output.is_err() {
        println!("{}", style("Error building pdf. Do you have pdflatex installed?").red().bold());
    }

    if true {
        let _ = Command::new("biblatex").args(args.clone()).output().unwrap();

        let _ = Command::new("pdflatex").args(args.clone()).output().unwrap();

        let _ = Command::new("pdflatex").args(args).output().unwrap();
    }

    if !debug {
        remove_files();
    }
}

pub fn count(project_path: PathBuf) {
    let build_settings = settings::find_and_parse_toml(project_path);

    let tex_file = build_settings.tex_filename();

    let args = vec![tex_file.as_str()];

    let output = Command::new("texcount").args(args).output();

    if output.is_err() {
        println!("{}", style("Error running Texcount. Do you have texcount installed?").red().bold());
    }

    let output = output.expect("Could not run texcount").stdout;

    let output = String::from_utf8(output).unwrap();

    println!("{}", output);
}

pub fn remove_files() {
    let out_folder_path = PathBuf::from("./out");
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

        if !ignore_file(filename) && path.is_file() && std::fs::remove_file(path).is_err() {
            println!("{}", style("failed to remove file in out folder").red().bold());
        }
    }
}

fn ignore_file(filename: &str) -> bool {
    let binding = PathBuf::from(filename);
    let extension = binding.extension().unwrap().to_str().unwrap();

    if extension == "pdf" {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ignore_file() {
        assert_eq!(super::ignore_file("main.pdf"), true);
        assert_eq!(super::ignore_file("main.log"), false);
    }
}
