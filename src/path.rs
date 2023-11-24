//! Contains utilities for working with paths

use crate::prelude::*;
use std::path::{Path, PathBuf};

/// Takes an optional path to change to, otherwise change to the closest parent directory with an itex-build.toml
pub fn change_to_itex_path(path: Option<PathBuf>) -> PathBuf {
    let og_path = unwrap_result!(std::env::current_dir(), "Failed to get current working directory");

    let e = std::env::set_current_dir(match path {
        Some(p) => p,
        None => {
            unwrap_result!(find_itex_path(), "Cannot find itex-build.toml in this or any parent directories")
        }
    });
    unwrap_result!(e, "Failed to change directory");

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

        let e = current_folder.parent();

        current_folder = unwrap_option!(
            e,
            "Failed to find parent folder",
            "Failed to find parent folder of: {}",
            current_folder.display()
        );
    }

    Result::Err(())
}

fn folder_contains(filename: &std::ffi::OsStr, folder: &Path) -> bool {
    let read_dir = unwrap_result!(
        std::fs::read_dir(folder),
        "Failed to read directory",
        "Failed to read directory: {}",
        folder.display()
    );

    for object in read_dir {
        let object = unwrap_result!(
            object,
            "Failed to read object in directory",
            "Failed to read object in directory: {}",
            folder.display()
        );
        if object.file_name() == filename {
            return true;
        }
    }

    false
}
