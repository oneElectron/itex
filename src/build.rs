use crate::prelude::*;
use std::io::{stdout, Write};

pub fn build(debug: bool, draft_mode: bool) {
    let mut settings = Settings::find_and_parse_toml();
    settings.ensure_build_artifacts_path_exists();
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

    if pdflatex_output.status.success() {
        copy_pdf_to_out_dir(&settings);
    }
}

fn copy_pdf_to_out_dir(settings: &Settings) {
    let pdf_path = settings
        .build_artifacts_path()
        .join(settings.tex_filename().replace(".tex", ".pdf"));
    let target_pdf_path = settings.output_dir().join(settings.tex_filename().replace(".tex", ".pdf"));

    log::info!("Copying {} to {}", pdf_path.display(), target_pdf_path.display());

    std::fs::copy(&pdf_path, &target_pdf_path).unwrap();
}
