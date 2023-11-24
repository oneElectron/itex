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
        if exe_path.is_none() {
            println!(
                "{} Do you have bib installed and in your PATH?\nIf not you can install TexLive from: https://miktex.org",
                style("Error running bibtex.").red().bold()
            );
            exit!(1);
        }

        let aux_path = format!(
            "./{}/{}.aux",
            settings.build_artifacts_path().to_string_lossy(),
            settings.tex_filename_without_extension()
        );

        Self {
            exe_path: exe_path.unwrap(),
            args: vec![aux_path],
        }
    }

    fn run(&self, _print_errors: bool) -> std::process::Output {
        let output = std::process::Command::new(self.exe_path.clone()).args(self.args.clone()).output();

        if output.is_err() {
            println!(
                "{} Do you have bibtex installed and in your PATH?",
                style("Error running bibtex.").red().bold()
            );
        }

        unwrap_result!(output, "Failed to read output of pdflatex")
    }

    fn set_executable_path(&mut self, path: PathBuf) {
        if path.is_file() {
            self.exe_path = path;
        } else {
            self.exe_path = path.find_in_path().unwrap_or_else(|| {
                println!(
                    "{}  Do you have bibtex installed and in your PATH?",
                    style("Error running bibtex.").red().bold()
                );

                exit!(1);
            });
        }
    }
}
