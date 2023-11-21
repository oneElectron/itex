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
        if exe_path.is_none() {
            println!(
                "{} Do you have pdflatex installed and in your PATH?\nIf not you can install TexLive from: https://miktex.org",
                style("Error running pdflatex.").red().bold()
            );
            exit!(1);
        }

        let output_dir = format!("-output-directory=./{}/", settings.build_artifacts_path().to_string_lossy());

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
                "{} Do you have pdflatex installed and in your PATH?\nIf not you can install TexLive from: https://miktex.org",
                style("Error running pdflatex.").red().bold()
            );
        }

        let output = unwrap_result!(output, "Failed to read output of pdflatex");

        output
    }

    fn set_executable_path(&mut self, path: PathBuf) {
        if path.is_file() {
            self.exe_path = path;
        } else {
            self.exe_path = path.find_in_path().unwrap_or_else(|| {
                println!(
                    "{} Do you have pdflatex installed and in your PATH?\nIf not you can install TexLive from: https://miktex.org",
                    style("Error running pdflatex.").red().bold()
                );

                exit!(1);
            });
        }
    }
}
