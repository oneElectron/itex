use super::Error;
use std::{path::PathBuf, process::Command, result::Result, string::String};

#[cfg(unix)]
pub(super) fn search_in_homebrew() -> Result<PathBuf, Error> {
    if Command::new("brew").output().is_err() {
        return Err(Error::NotFound);
    }
    let cellar_path = String::from_utf8(Command::new("brew").arg("--prefix").output().unwrap().stdout.to_vec());
    if cellar_path.is_err() {
        println!("{}", console::style("Failed to run brew --prefix and read the output"));
        return Err(Error::NotFound);
    }

    let mut cellar_path = PathBuf::from(cellar_path.unwrap().trim());
    cellar_path.push("etc");
    cellar_path.push("itex");
    cellar_path.push("itex-templates");

    if !cellar_path.is_dir() {
        return Err(Error::NotFound);
    }

    Ok(cellar_path)
}

#[cfg(unix)]
#[cfg(feature = "updater")]
pub(super) fn search_in_unix() -> Result<PathBuf, Error> {
    let home = std::env::var("HOME").expect("Could not find home");

    let path = PathBuf::from(home + "/.local/share/itex/itex-templates");
    if path.is_dir() {
        #[cfg(feature = "updater")]
        crate::updater::version_check(&path);
        Ok(path)
    } else {
        Err(Error::NotFound)
    }
}

#[cfg(windows)]
pub(super) fn search_in_windows() -> Result<PathBuf, Error> {
    let mut app_data_dir = PathBuf::from(std::env::var("LOCALAPPDATA").expect("No App Data dir found"));

    app_data_dir.push("itex");
    app_data_dir.push("itex-templates");
    if app_data_dir.is_dir() {
        #[cfg(feature = "updater")]
        crate::updater::version_check(&app_data_dir);
        Ok(app_data_dir)
    } else {
        Err(Error::NotFound)
    }
}
