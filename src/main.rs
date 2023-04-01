mod builder;
mod init;
mod macros;
mod options;
mod parser;
mod updater;

use console::style;
use init::copy_template;
use parser::parse_options;
use parser::Command;
use std::env;

fn main() {
    let command = parse_options(env::args().collect());

    if let Command::Init(template_name, _, output_path, disable_os_search) = command {
        let output_path = match output_path {
            None => std::env::current_dir().expect("could not find pwd"),
            Some(p) => p,
        };

        copy_template(template_name.replace('\n', ""), output_path, disable_os_search);

        let out_folder = env::current_dir();
        if out_folder.is_err() {
            println!("{}", style("Could not find current path").red().bold());
        }

        let mut out_folder = out_folder.unwrap();
        out_folder.push("out");
        if !out_folder.is_dir() && std::fs::create_dir(out_folder).is_err() {
            println!("{}", style("failed to create out folder").red().bold());
        }
    } else if let Command::List(disable_os_search) = command {
        init::list_template_names(disable_os_search);
    } else if let Command::Info(name, disable_os_search) = command {
        init::get_template_info(name, disable_os_search);
    } else if let Command::Build(debug) = command {
        builder::build(debug);
    } else if let Command::Count = command {
        builder::count();
    } else if let Command::Clean = command {
        builder::remove_files();
    } else if let Command::Options(setting) = command {
        options::handle_options(setting);
    }
}
