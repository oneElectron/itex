use crate::prelude::*;
use std::path::PathBuf;

#[allow(dead_code)]
pub enum CopyFilesExitCode {
    SomeFilesExist,
    AllFilesExist,
}

pub fn copy_files(from: PathBuf, dry_run: bool, info: &crate::info::TemplateInfo) -> Result<(), CopyFilesExitCode> {
    let template_folder = std::fs::read_dir(from);
    let template_folder = unwrap_result!(template_folder, "Could not read from templates folder");

    let pwd = std::env::current_dir().unwrap();

    for file in template_folder {
        let file = unwrap_result!(file, "Could not read file");

        if should_ignore_file(file.file_name().to_string_lossy().to_string(), info) {
            continue;
        }

        let file_path = pwd.clone().join(file.file_name());

        log::trace!("Copying file: {}, is dry run: {}", file.file_name().to_string_lossy(), dry_run);

        if dry_run && file_path.exists() {
            return Err(CopyFilesExitCode::SomeFilesExist);
        } else {
            unwrap_result!(std::fs::copy(file.path(), file_path.clone()), "Could not copy file");
        }
    }

    Ok(())
}

fn should_ignore_file(filename: String, info: &TemplateInfo) -> bool {
    let filename = filename.trim().to_owned();
    if &filename == "itex-info.toml" {
        return true;
    }

    if let Some(excluded_files) = info.excluded_files.clone() {
        if excluded_files.contains(&filename) {
            return true;
        }
    }

    false
}
