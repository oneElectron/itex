use crate::prelude::*;
use std::io::{stdout, Write};
use std::path::PathBuf;

pub fn build(debug: bool, project_path: PathBuf) {
    let settings = Settings::find_and_parse_toml();
    let pdflatex = PDFLatex::from_settings(settings.clone());
    let bibtex = Bibtex::from_settings(settings.clone());

    settings.check_tex_file_exists();

    let pdflatex_output = pdflatex.run();
    if debug {
        stdout().write_all(&pdflatex_output.stdout).unwrap();
    }
    if settings.compile_bib(None) {
        let bibtex_output = bibtex.run();
        pdflatex.run();
        pdflatex.run();
        if debug {
            stdout().write_all(&bibtex_output.stdout).unwrap();
        }
    }

    if !debug {
        clean(project_path);
    }
}
