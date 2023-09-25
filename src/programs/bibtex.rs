use crate::prelude::*;
use console::style;
use std::path::PathBuf;

pub struct Bibtex {
    exe_path: PathBuf,
    args: Vec<String>,
}

impl Executable for Bibtex {
    #[allow(unused_variables)]
    fn from_settings(settings: crate::Settings) -> Self {
        let exe_path = PathBuf::find_in_path(PathBuf::from("bibtex"));
        // let aux_path = format!("./out/{}.aux", settings.tex_filename().split(".")[0..-1]); fixme
        let aux_path = "./out/main.aux".to_string();

        Self {
            exe_path: exe_path.unwrap(),
            args: vec![aux_path],
        }
    }

    fn run(&self) -> std::process::Output {
        let output = std::process::Command::new(self.exe_path.clone()).args(self.args.clone()).output();

        if output.is_err() {
            println!(
                "{}",
                style("Error running bibtex. Do you have bibtex installed and in your PATH?")
                    .red()
                    .bold()
            );
        }

        output.unwrap()
    }

    fn set_executable_path(&mut self, path: PathBuf) {
        if path.is_file() {
            self.exe_path = path;
        } else {
            self.exe_path = PathBuf::find_in_path(path.as_path()).unwrap();
        }
    }
}

impl PDFLatex {}
