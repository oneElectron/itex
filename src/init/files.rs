use log::trace;
use std::{fs, fs::read_dir, path::Path, path::PathBuf};

#[allow(dead_code)]
pub enum CopyFilesExitCode {
    SomeFilesExist,
    AllFilesExist,
    Other,
}

pub fn copy_files(from: PathBuf, to: &Path, dry_run: bool) -> Result<isize, CopyFilesExitCode> {
    let template_files = read_dir(from).expect("could not find template folder");

    let mut end: bool = false;

    for file in template_files {
        let file = file.unwrap().path();

        let pwd = to.to_owned().clone();

        if dry_run && pwd.with_file_name(file.file_name().unwrap().to_str().unwrap()).exists() {
            println!("file exists: {}", file.file_name().unwrap().to_str().unwrap());
            end = true;
        }

        trace!(
            "pwd = {}",
            pwd.clone()
                .with_file_name(file.clone().file_name().unwrap().to_str().unwrap())
                .to_str()
                .unwrap()
        );

        if file.clone().file_name().unwrap().to_str().unwrap() != "itex-info.toml"
            && file.clone().file_name().unwrap().to_str().unwrap() != "Makefile"
            && !dry_run
            && fs::copy(&file, pwd.with_file_name(file.file_name().unwrap().to_str().unwrap())).is_err()
        {
            println!("Error copying file");
        }
    }

    if end {
        return Err(CopyFilesExitCode::SomeFilesExist);
    }

    Ok(0)
}
