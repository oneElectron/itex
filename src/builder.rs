use super::exit;
use console::style;
use serde::Deserialize;
use std::path::PathBuf;
use std::process::Command;
use std::str;

#[derive(Deserialize, Debug)]
struct BuildOptions {
    default_name: String,
    tex_file: Option<String>,
}

pub fn build(debug: bool) {
    let build_options = find_and_parse_toml();

    let tex_file = build_options.tex_file.unwrap_or_else(|| build_options.default_name + ".tex");

    let args = vec!["-output-directory", "./out/", tex_file.as_str()];

    let output = Command::new("pdflatex").args(args).output();

    if output.is_err() {
        println!("{}", style("Error building pdf. Do you have pdflatex installed?").red().bold());
    }

    if !debug {
        remove_files();
    }
}

pub fn count() {
    let output = Command::new("texcount").arg("main.tex").output();

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

fn find_and_parse_toml() -> BuildOptions {
    let toml_file: PathBuf = if PathBuf::from("./itex-build.toml").is_file() {
        PathBuf::from("./itex-build.toml")
    } else if PathBuf::from("./.itex-build.toml").is_file() {
        PathBuf::from("./.itex-build.toml")
    } else {
        println!("{}", style("No itex build file found, please create one.").red().bold());
        exit!(0);
    };

    let build_file = std::fs::read_to_string(toml_file);
    if build_file.is_err() {
        println!("{}", style("failed to read itex build file").red().bold());
    }
    let build_file = build_file.unwrap();

    let build_toml: BuildOptions = toml::from_str(build_file.as_str()).unwrap();

    build_toml
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ignore_file() {
        assert_eq!(super::ignore_file("main.pdf"), true);
        assert_eq!(super::ignore_file("main.log"), false);
    }
}
