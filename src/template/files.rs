use std::{
    fs::read_dir,
    path::PathBuf,
    env
};

#[allow(dead_code)]
pub enum CopyFilesExitCode {
    SomeFilesExist,
    AllFilesExist,
    Other
}

pub fn copy_files(from: PathBuf, dry_run: bool) -> Result<isize, CopyFilesExitCode> {

    let template_files = read_dir(from).expect("could not find template folder");

    let mut end: bool = false;
    
    for file in template_files {
        let file = PathBuf::from(file.unwrap().path());

        let mut pwd = env::current_dir().unwrap();
        pwd.push("file.txt");

        if pwd.with_file_name(&file.file_name().unwrap().to_str().unwrap()).exists() {
            println!(
                "file exists: {}",
                file.file_name().unwrap().to_str().unwrap()
            );
            end = true;
        }

        if ! dry_run {
            if std::fs::copy(&file, pwd.with_file_name(file.file_name().unwrap().to_str().unwrap())).is_err() {
                println!("Error copying file");
            }
        }
    }

    if end {
        return Err(CopyFilesExitCode::SomeFilesExist)
    }

    Ok(0)
}