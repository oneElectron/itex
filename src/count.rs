use crate::prelude::*;
use std::io::Write;
use std::path::PathBuf;

pub fn count(project_path: PathBuf) {
    let build_settings = Settings::find_and_parse_toml(&project_path);

    let texcount = Texcount::from_settings(build_settings);

    let output = texcount.run();

    std::io::stdout().write_all(&output.stdout).unwrap();
}
