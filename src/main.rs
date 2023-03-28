mod builder;
mod init;
mod runtime_helper;
mod updater;

use init::copy_template;
use runtime_helper::parse_options;
use runtime_helper::Command;
use std::env;

fn main() {
    let command = parse_options(env::args().collect());

    if let Command::Init(template_name, _, output_path, disable_os_search) = command {
        let output_path = match output_path {
            None => std::path::PathBuf::from("."),
            Some(p) => p,
        };

        copy_template(
            template_name.replace('\n', ""),
            output_path,
            disable_os_search,
        );

        let mut out_folder = env::current_dir().expect("Could not find current path");
        out_folder.push("out");
        if !out_folder.is_dir() {
            std::fs::create_dir(out_folder).expect("failed to create out folder");
        }
    } else if let Command::List(disable_os_search) = command {
        init::list_template_names(disable_os_search);
    } else if let Command::Info(name, disable_os_search) = command {
        init::get_template_info(name, disable_os_search);
    } else if let Command::Build(debug) = command {
        builder::build(debug);
    }
}
