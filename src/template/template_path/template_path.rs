use std::{path::PathBuf, process::Command, result::Result, string::String, env};

pub fn search_in_homebrew() -> Result<PathBuf, i32> {
    let cellar_path = String::from_utf8(
        Command::new("brew")
            .arg("--prefix")
            .output()
            .unwrap()
            .stdout
            .to_vec(),
    );
    if cellar_path.is_err() {
        eprintln!("Failed to run brew --prefix and read the output");
        return Err(0);
    }

    let mut cellar_path = PathBuf::from(cellar_path.unwrap().trim());
    cellar_path.push("etc");
    cellar_path.push("itex");
    cellar_path.push("itex-templates");

    if !cellar_path.is_dir() {
        return Err(1);
    }

    return Ok(cellar_path);
}

pub fn search_in_unix() -> Result<PathBuf, i32> {
    let home = env::var("HOME")
        .expect("Could not find home");

    let path = PathBuf::from(home + "/.local/share/itex/itex-templates");
    if path.is_dir() {
        return Ok(path)
    }

    return Err(0)
}

pub fn search_in_windows() -> Result<PathBuf, i32> {
    let mut app_data_dir =
        PathBuf::from(std::env::var("LOCALAPPDATA").expect("No App Data dir found"));

    app_data_dir.push("itex");
    app_data_dir.push("itex-templates");
    if app_data_dir.is_dir() {
        return Ok(app_data_dir);
    } else {
        return Err(0);
    }
}
