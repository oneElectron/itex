use crate::prelude::*;
use std::io::Write;

/// Count
pub fn count() {
    let settings = Settings::from_global();
    settings.check_tex_filename_is_set();

    let texcount = Texcount::from_settings(settings);

    let output = texcount.run(true);

    std::io::stdout().write_all(&output.stdout).unwrap();
}
