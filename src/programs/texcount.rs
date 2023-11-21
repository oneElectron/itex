use crate::prelude::*;
use console::style;
use std::io::Write;
use std::path::PathBuf;

pub struct Texcount {
    exe_path: PathBuf,
    args: Vec<String>,
}

impl Executable for Texcount {
    fn from_settings(settings: crate::Settings) -> Self {
        let tex_filename = settings.tex_filename();
        let exe_path = "texcount".find_in_path();
        if exe_path.is_none() {
            println!(
                "{} Do you have texcount installed and in your PATH?",
                style("Error running texcount.").red().bold()
            );
            exit!(1);
        }

        Self {
            exe_path: exe_path.unwrap(),
            args: vec![tex_filename],
        }
    }

    fn run(&self) -> std::process::Output {
        let output = std::process::Command::new(self.exe_path.clone()).args(self.args.clone()).output();

        if output.is_err() {
            println!(
                "{} Do you have texcount installed and in your PATH?",
                style("Error running texcount.").red().bold()
            );
        }

        let output = unwrap_result!(output, "Failed to read output of pdflatex");

        if !output.status.success() {
            println!("{}", style("Undefined error running texcount"));

            std::io::stdout().write_all(&output.stderr).unwrap();

            exit!(1);
        }

        output
    }

    fn set_executable_path(&mut self, path: PathBuf) {
        if path.is_file() {
            self.exe_path = path;
        } else {
            self.exe_path = path.find_in_path().unwrap_or_else(|| {
                println!(
                    "{}  Do you have texcount installed and in your PATH?",
                    style("Error running texcount.").red().bold()
                );

                exit!(1);
            });
        }
    }
}