mod template;
mod runtime_helper;

use template::copy_template;
use runtime_helper::parse_options;
use std::{
    process::exit,
    io,
    env::{
        current_dir,
        args,
    }
};

fn main() -> io::Result<()> {
    let opts = parse_options(std::env::args().collect());

    if opts.list_templates { // list templates and exit
        template::list_template_names(opts.debug);
        exit(0);
    }

    // copy template
    copy_template(
        opts.template_name
            .clone()
            .replace("\n", "")
            .to_string(), 
        opts.debug, 
        opts.disable_os_search);
    
    // Create out folder
    let mut out_folder = current_dir().unwrap();
    out_folder.push("out");
    if !out_folder.is_dir() {
    std::fs::create_dir(out_folder)
        .expect("failed to create out folder");
    }

    Ok(())
}
