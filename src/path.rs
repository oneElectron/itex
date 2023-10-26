//! Contains utilities for working with paths

use crate::prelude::*;
use console::style;
use std::path::{Path, PathBuf};

/// Takes an optional path to change to, otherwise change to the closest parent directory with an itex-build.toml
pub fn change_to_itex_path(path: Option<PathBuf>) -> PathBuf {
    let og_path = std::env::current_dir().unwrap();
    std::env::set_current_dir(match path {
        Some(p) => p,
        None => {
            let p = find_itex_path();
            if p.is_err() {
                println!(
                    "{}",
                    style("Cannot find itex-build.toml in this or any parent directories").red().bold()
                );
                exit!(0);
            }

            p.unwrap()
        }
    })
    .unwrap();

    og_path
}

/// This function searches for the closest parent folder with an itex-build.toml (including the current folder)
/// # Limitations
/// This function will not check ```/``` for an itex-build.toml
pub fn find_itex_path() -> Result<PathBuf, ()> {
    let filename = std::ffi::OsString::from("itex-build.toml");
    let slash = PathBuf::from("/");

    let pwd = std::env::current_dir().unwrap();
    let mut current_folder: &Path = pwd.as_path();

    while current_folder != slash {
        if folder_contains(filename.as_os_str(), current_folder) {
            return Ok(PathBuf::from(current_folder));
        }

        current_folder = current_folder.parent().unwrap();
    }

    Result::Err(())
}

fn folder_contains(filename: &std::ffi::OsStr, folder: &Path) -> bool {
    let read_dir = std::fs::read_dir(folder).unwrap();

    for object in read_dir {
        let object = object.unwrap();
        if object.file_name() == filename {
            return true;
        }
    }

    false
}
