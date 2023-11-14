use crate::prelude::*;
use std::io::{stdout, Write};
use std::path::PathBuf;

pub fn build(debug: bool, draft_mode: bool, project_path: PathBuf) {
    let mut settings = Settings::find_and_parse_toml();
    settings.check_tex_filename_is_set();

    if draft_mode {
        settings.set_draft_mode(Some(true));
    }

    let pdflatex = PDFLatex::from_settings(settings.clone());
    let bibtex = Bibtex::from_settings(settings.clone());

    pdflatex.run();
    let (bibtex_output, _) = bibtex.run();
    pdflatex.run();
    let (pdflatex_output, _) = pdflatex.run();

    if debug || !pdflatex_output.status.success() {
        println!("{}", console::style("--- Bibtex Output ---").blue().bold());
        stdout().write_all(&bibtex_output.stdout).unwrap();
        println!("{}", console::style("--- PDFLatex Output ---").blue().bold());
        stdout().write_all(&pdflatex_output.stdout).unwrap();
    }

    if settings.clean() && !debug && pdflatex_output.status.success() {
        clean(project_path, settings);
    }
}
