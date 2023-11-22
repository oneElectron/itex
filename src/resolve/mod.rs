//! Resolve template paths
mod os_search;

use crate::prelude::*;
use console::style;
use os_search::*;
use std::{env, path::PathBuf, result::Result};

enum Error {
    NotFound,
    #[cfg(feature = "updater")]
    NotDownloaded,
    Undefined,
}

/// Resolve template name
/// This function, unlike ```resolve_templates_folder``` will handle all errors and will exit the program if one is encountered
pub fn resolve_template(template_name: &str, disable_os_search: bool, search_path: &Option<PathBuf>) -> PathBuf {
    let mut path: PathBuf = match internal_resolve_templates_folder(disable_os_search, search_path) {
        Ok(p) => p,
        Err(Error::NotFound) => {
            println!("{}", style("Templates could not be found").red().bold());
            exit!(0);
        }
        #[cfg(feature = "updater")]
        Err(Error::NotDownloaded) => {
            println!("{}", style("Templates could not be found.").red().bold());
            crate::updater::download_templates(true, true);
            exit!(0);
        }
        Err(Error::Undefined) => {
            println!(
                "{}",
                style("An undefined error occurred trying to find the templates folder")
                    .red()
                    .bold()
            );
            exit!(0);
        }
    };

    path.push(template_name);

    if !path.is_dir() {
        println!(
            "{} {}",
            console::style("Could not find the template:").red().bold(),
            style(template_name).red().bold()
        );
    }

    path
}

/// Resolve itex-templates folder
pub fn resolve_templates_folder(disable_os_search: bool, search_path: &Option<PathBuf>) -> PathBuf {
    match internal_resolve_templates_folder(disable_os_search, search_path) {
        Ok(p) => p,
        Err(Error::NotFound) => {
            exit!(0);
        }
        #[cfg(feature = "updater")]
        Err(Error::NotDownloaded) => {
            exit!(0);
        }
        Err(Error::Undefined) => {
            exit!(0);
        }
    }
}

/// Resolves to the path to the templates folder
fn internal_resolve_templates_folder(disable_os_search: bool, search_path: &Option<PathBuf>) -> Result<PathBuf, Error> {
    // Search in search_path
    if let Some(search_path) = search_path {
        if search_path.is_dir() {
            return Ok(search_path.clone());
        }
    }

    if !disable_os_search {
        #[cfg(unix)]
        {
            // if OS is UNIX
            #[cfg(feature = "updater")]
            if let Ok(path_to_templates) = search_in_unix() {
                return Ok(path_to_templates);
            }
            if let Ok(path_to_templates) = search_in_homebrew() {
                return Ok(path_to_templates);
            }

            #[cfg(feature = "updater")]
            return Err(Error::NotDownloaded);
            #[cfg(not(feature = "updater"))]
            return Err(Error::NotFound);
        }
        #[cfg(windows)]
        {
            // if OS is Windows
            if let Ok(path_to_templates) = search_in_windows() {
                return Ok(path_to_templates);
            }
            #[cfg(feature = "updater")]
            return Err(Error::NotDownloaded);
            #[cfg(not(feature = "updater"))]
            return Err(Error::NotFound);
        }
    }

    // search current directory
    let pwd = env::current_dir();
    let mut pwd = pwd.unwrap();
    pwd.push("itex-templates");
    if pwd.is_dir() {
        return Ok(pwd);
    }
    drop(pwd);

    // search in ..
    let mut previous_dir = env::current_dir().unwrap().parent().unwrap().to_path_buf();
    previous_dir.push("itex-templates");
    if previous_dir.is_dir() {
        return Ok(previous_dir);
    }

    Err(Error::Undefined)
}
