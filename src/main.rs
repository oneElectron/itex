mod template;
mod runtime_helper;
mod template_updater;

use template::copy_template;
use runtime_helper::parse_options;
use std::{
    process::exit,
    env
};

fn main() {
    let opts = parse_options(env::args().collect());

    if opts.list_templates {
        template::list_template_names();
        exit(0);
    }

    if opts.update {
        template_updater::download_templates();
        exit(0);
    }

    copy_template(
        opts.template_name
            .replace("\n", ""),
        opts
    );
    
    let mut out_folder = env::current_dir()
        .expect("Could not find current path");

    out_folder.push("out");
    if !out_folder.is_dir() {
        std::fs::create_dir(out_folder)
            .expect("failed to create out folder");
    }
}
