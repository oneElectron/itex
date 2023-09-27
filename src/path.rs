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

pub trait FindInPath<T> {
    fn find_in_path(file: T) -> Option<Self>
    where
        Self: Sized;
}

impl FindInPath<String> for PathBuf {
    /// Finds ```file``` in the users PATH
    // Most of this code was shamelessly copied from sudo-rs: https://github.com/memorysafety/sudo-rs/blob/b5eb2c654f8971a7191d341228f06d58ba3746ff/src/common/resolve.rs#L132
    fn find_in_path(command: String) -> Option<Self> {
        // To prevent command spoofing, sudo checks "." and "" (both denoting current directory)
        // last when searching for a command in the user's PATH (if one or both are in the PATH).
        // Depending on the security policy, the user's PATH environment variable may be modified,
        // replaced, or passed unchanged to the program that sudo executes.
        let mut resolve_current_path = false;

        std::env::var("PATH")
            .unwrap()
            .split(':')
            // register whether to look in the current directory, but first check the other PATH segments
            .filter(|&path| {
                if path.is_empty() || path == "." {
                    resolve_current_path = true;

                    false
                } else {
                    true
                }
            })
            .map(|path| PathBuf::from(path).join(command.clone()))
            .find(is_valid_executable)
            .or_else(|| {
                if resolve_current_path {
                    std::env::current_dir()
                        .ok()
                        .map(|dir| dir.join(command))
                        .and_then(|path| Some(path))
                } else {
                    None
                }
            })
    }
}

impl FindInPath<&Path> for PathBuf {
    /// Finds ```file``` in the users PATH
    // Most of this code was shamelessly copied from sudo-rs: https://github.com/memorysafety/sudo-rs/blob/b5eb2c654f8971a7191d341228f06d58ba3746ff/src/common/resolve.rs#L132
    fn find_in_path(command: &Path) -> Option<Self> {
        // To prevent command spoofing, sudo checks "." and "" (both denoting current directory)
        // last when searching for a command in the user's PATH (if one or both are in the PATH).
        // Depending on the security policy, the user's PATH environment variable may be modified,
        // replaced, or passed unchanged to the program that sudo executes.
        let mut resolve_current_path = false;

        std::env::var("PATH")
            .unwrap()
            .split(":")
            // register whether to look in the current directory, but first check the other PATH segments
            .filter(|&path| {
                if path.is_empty() || path == "." {
                    resolve_current_path = true;

                    false
                } else {
                    true
                }
            })
            .map(|path| PathBuf::from(path).join(command))
            .find(is_valid_executable)
            .or_else(|| {
                if resolve_current_path {
                    std::env::current_dir()
                        .ok()
                        .map(|dir| dir.join(command))
                        .and_then(|path| Some(path))
                } else {
                    None
                }
            })
    }
}

impl FindInPath<PathBuf> for PathBuf {
    /// Finds ```file``` in the users PATH
    // Most of this code was shamelessly copied from sudo-rs: https://github.com/memorysafety/sudo-rs/blob/b5eb2c654f8971a7191d341228f06d58ba3746ff/src/common/resolve.rs#L132
    fn find_in_path(command: PathBuf) -> Option<Self> {
        // To prevent command spoofing, sudo checks "." and "" (both denoting current directory)
        // last when searching for a command in the user's PATH (if one or both are in the PATH).
        // Depending on the security policy, the user's PATH environment variable may be modified,
        // replaced, or passed unchanged to the program that sudo executes.
        let mut resolve_current_path = false;

        std::env::var("PATH")
            .unwrap()
            .split(":")
            // register whether to look in the current directory, but first check the other PATH segments
            .filter(|&path| {
                if path.is_empty() || path == "." {
                    resolve_current_path = true;

                    false
                } else {
                    true
                }
            })
            .map(|path| PathBuf::from(path).join(command.clone()))
            .find(is_valid_executable)
            .or_else(|| {
                if resolve_current_path {
                    std::env::current_dir()
                        .ok()
                        .map(|dir| dir.join(command))
                        .and_then(|path| Some(path))
                } else {
                    None
                }
            })
    }
}

fn is_valid_executable(path: &PathBuf) -> bool {
    if path.is_file() {
        return true; // We don't care
    }

    false
}
