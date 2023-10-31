use crate::prelude::*;
use std::io::Write;

/// Count
pub fn count() {
    let build_settings = Settings::find_and_parse_toml();

    let texcount = Texcount::from_settings(build_settings);

    let (output, _) = texcount.run();

    std::io::stdout().write_all(&output.stdout).unwrap();
}
