//! Provides structs as an abstraction to real programs in order to make running said programs easier

mod bibtex;
mod pdflatex;
mod texcount;

pub use bibtex::Bibtex;
pub use pdflatex::PDFLatex;
pub use texcount::Texcount;

use std::path::PathBuf;

pub trait Executable {
    fn from_settings(settings: crate::Settings) -> Self;
    fn run(&self) -> std::process::Output;
    fn set_executable_path(&mut self, path: PathBuf);
}
