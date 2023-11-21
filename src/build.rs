use crate::clean::clean_build_artifacts_folder;
use crate::prelude::*;

use std::io::{stdout, Write};

pub fn build(debug: bool, draft_mode: bool, settings: Option<Settings>) {
    let mut settings = settings.unwrap_or(Settings::from_global());

    settings.check_tex_filename_is_set();
    settings.ensure_build_artifacts_path_exists();

    if draft_mode {
        settings.set_draft_mode(Some(true));
    }

    let pdflatex = PDFLatex::from_settings(settings.clone());
    let bibtex = Bibtex::from_settings(settings.clone());

    pdflatex.run(false);
    let bibtex_output = bibtex.run(true);
    pdflatex.run(false);
    let pdflatex_output = pdflatex.run(true);

    if debug || settings.debug() {
        println!("{}", console::style("--- PDFLatex Output ---").blue().bold());
        stdout().write_all(&pdflatex_output.stdout).unwrap();
        println!("{}", console::style("--- Bibtex Output ---").blue().bold());
        stdout().write_all(&bibtex_output.stdout).unwrap();
    }

    if settings.clean() {
        clean_build_artifacts_folder(&settings);
    }

    if pdflatex_output.status.success() {
        copy_pdf_to_out_dir(&settings);
    }
}

pub fn safe_build() {
    let mut settings = Settings::from_global();
    settings.set_clean(Some(false));
    settings.set_compile_bib(Some(true));
    settings.set_draft_mode(Some(false));

    clean_build_artifacts_folder(&settings);

    build(true, false, Some(settings.clone()));

    build(true, false, Some(settings.clone()));
}

fn copy_pdf_to_out_dir(settings: &Settings) {
    let pdf_path = settings
        .build_artifacts_path()
        .join(settings.tex_filename().replace(".tex", ".pdf"));
    let target_pdf_path = settings.output_dir().join(settings.tex_filename().replace(".tex", ".pdf"));

    log::info!("Copying {} to {}", pdf_path.display(), target_pdf_path.display());

    std::fs::copy(&pdf_path, &target_pdf_path).unwrap();
}
