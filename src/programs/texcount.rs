use crate::prelude::*;
use console::style;
use std::path::PathBuf;

pub struct Texcount {
    exe_path: PathBuf,
    args: Vec<String>,
}

impl Executable for Texcount {
    fn from_settings(settings: crate::Settings) -> Self {
        let tex_filename = settings.tex_filename();
        let exe_path = "texcount".find_in_path();

        Self {
            exe_path: exe_path.unwrap(),
            args: vec![tex_filename],
        }
    }

    fn run(&self) -> std::process::Output {
        let output = std::process::Command::new(self.exe_path.clone()).args(self.args.clone()).output();

        if output.is_err() {
            println!(
                "{}",
                style("Error running texcount. Do you have texcount installed and in your PATH?")
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
                    style("Error running texcount. Do you have texcount installed and in your PATH?")
                        .red()
                        .bold()
                );

                exit!(1);
            });
        }
    }
}
