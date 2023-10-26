use crate::prelude::*;
use console::style;
use std::path::PathBuf;

pub struct Bibtex {
    exe_path: PathBuf,
    args: Vec<String>,
}

impl Executable for Bibtex {
    fn from_settings(settings: crate::Settings) -> Self {
        let exe_path = "bibtex".find_in_path();
        let aux_path = format!(
            "./{}/{}.aux",
            settings.output_dir().to_string_lossy(),
            settings.tex_filename_without_extension()
        );

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
            self.exe_path = path.find_in_path().unwrap_or_else(|| {
                println!(
                    "{}",
                    style("Error running bibtex. Do you have bibtex installed and in your PATH?")
                        .red()
                        .bold()
                );

                exit!(1);
            });
        }
    }
}
