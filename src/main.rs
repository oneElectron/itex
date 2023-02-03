mod template;
mod runtime_helper;

use template::copy_template;
use runtime_helper::parse_options;
use std::{
    process::exit,
    io,
    env
};

fn main() -> io::Result<()> {
    let opts = parse_options(env::args().collect());

    if opts.list_templates { // list templates and exit
        template::list_template_names();
        exit(0);
    }

    // copy template
    copy_template(
        opts.template_name
            .replace("\n", ""),
        opts
    );
    
    // Create out folder
    let mut out_folder = env::current_dir().expect("Could not find current path");
    out_folder.push("out");
    if !out_folder.is_dir() {
        std::fs::create_dir(out_folder)
        .expect("failed to create out folder");
    }

    Ok(())
}
