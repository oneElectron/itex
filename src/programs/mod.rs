//! Provides structs as an abstraction to real programs in order to make running said programs easier

mod bibtex;
mod pdflatex;
mod texcount;

pub use bibtex::Bibtex;
pub use bibtex::BibtexError;

pub use pdflatex::PDFLatex;
pub use pdflatex::PDFLatexError;

pub use texcount::Texcount;
pub use texcount::TexcountError;

use std::path::PathBuf;

pub trait Executable {
    type Error;
    fn from_settings(settings: crate::Settings) -> Self;
    fn run(&self) -> (std::process::Output, Self::Error);
    fn set_executable_path(&mut self, path: PathBuf);
}
