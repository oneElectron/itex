use crate::prelude::*;
use std::io::Write;

/// Count
pub fn count() {
    let settings = Settings::find_and_parse_toml();
    settings.check_tex_filename_is_set();

    let texcount = Texcount::from_settings(settings);

    let (output, _) = texcount.run();

    std::io::stdout().write_all(&output.stdout).unwrap();
}
