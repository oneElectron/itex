use crate::prelude::*;
use console::style;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct PDFLatex {
    exe_path: PathBuf,
    args: Vec<String>,
}

impl Executable for PDFLatex {
    fn from_settings(settings: crate::Settings) -> Self {
        let tex_filename = settings.tex_filename();
        let exe_path = PathBuf::find_in_path(PathBuf::from("pdflatex"));

        let output_dir = "-output-directory=./out/".to_string();

        Self {
            exe_path: exe_path.unwrap(),
            args: vec![output_dir, tex_filename],
        }
    }

    fn run(&self) -> std::process::Output {
        let output = std::process::Command::new(self.exe_path.clone()).args(self.args.clone()).output();

        if output.is_err() {
            println!(
                "{}",
                style("Error running pdflatex. Do you have pdflatex installed and in your PATH?")
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
