mod runtime_helper;
mod template;
mod template_updater;

use runtime_helper::parse_options;
use std::{env, process::exit};
use template::copy_template;

fn main() {
    let opts = parse_options(env::args().collect());

    if opts.list_templates {
        template::list_template_names(opts.disable_os_search);
        exit(0);
    } else if opts.info {
        template::get_template_info(opts.template_name.clone(), opts.disable_os_search);
        exit(0);
    } else if opts.update {
        template_updater::download_templates();
        exit(0);
    } else {
        let output_path = match opts.output_path {
            None => std::path::PathBuf::from("."),
            Some(p) => p,
        };
        copy_template(
            opts.template_name.replace('\n', ""),
            output_path,
            opts.disable_os_search,
        );

        let mut out_folder = env::current_dir().expect("Could not find current path");

        out_folder.push("out");
        if !out_folder.is_dir() {
            std::fs::create_dir(out_folder).expect("failed to create out folder");
        }
    }
}
