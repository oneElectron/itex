use console::style;

use crate::prelude::*;
use std::io::{stdout, BufRead, Write};
use std::path::PathBuf;

pub fn build(debug: bool, draft_mode: bool, project_path: PathBuf) {
    let mut settings = Settings::find_and_parse_toml();
    if draft_mode {
        settings.set_draft_mode(Some(true));
    }

    let pdflatex = PDFLatex::from_settings(settings.clone());
    let bibtex = Bibtex::from_settings(settings.clone());

    settings.check_tex_file_exists();

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

    if !debug && pdflatex_output.status.success() {
        clean(project_path, settings);
    }

    println!("Searching for errors");

    // Error / Warning Parser
    for line_result in pdflatex_output.stdout.lines() {
        let line = line_result
            .expect("Error reading lines to find errors, oh the irony...")
            .to_lowercase();

        if line.contains("warning") {
            println!("{}", style(line.clone()).yellow().bold());
        }
        if line.contains("error") {
            println!("{}", style(line).red().bold());
        }
    }
}
