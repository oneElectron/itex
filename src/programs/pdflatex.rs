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
        let exe_path = "pdflatex".find_in_path();

        let output_dir = format!("-output-directory=./{}/", settings.output_dir().to_string_lossy());

        let mut args: Vec<String> = vec![output_dir, tex_filename];
        if settings.draft_mode() {
            args.insert(0, "-draftmode".to_string());
        }

        Self {
            exe_path: exe_path.unwrap(),
            args,
        }
    }

    fn run(&self) -> std::process::Output {
        let output = std::process::Command::new(self.exe_path.clone()).args(self.args.clone()).output();

        if output.is_err() {
            println!(
                "{}",
                style("Error running pdflatex. Do you have pdflatex installed and in your PATH?\nIf not you can install TexLive from: <Insert URL here>")
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
            self.exe_path = path.find_in_path().unwrap_or_else(||{
                println!(
                    "{}",
                    style("Error running pdflatex. Do you have pdflatex installed and in your PATH?\nIf not you can install TexLive from: <Insert URL here>")
                    .red()
                    .bold()
                );

                exit!(1);
            });
        }
    }
}
