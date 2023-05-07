mod builder;
mod cli;
mod init;
mod macros;
mod settings;
mod updater;

use cli::{Cli, Parser};
use console::style;
use init::copy_template;
use std::env;
use std::path::PathBuf;

fn main() {
    let args = Cli::parse();

    if let cli::Commands::Init {
        name,
        output_path,
        search_path: _search_path,
        disable_os_search,
    } = args.command
    {
        let output_path = match output_path {
            None => std::env::current_dir().expect("could not find pwd"),
            Some(p) => PathBuf::from(p),
        };

        copy_template(name.replace('\n', ""), output_path, disable_os_search);

        let out_folder = env::current_dir();
        if out_folder.is_err() {
            println!("{}", style("Could not find current path").red().bold());
        }

        let mut out_folder = out_folder.unwrap();
        out_folder.push("out");
        if !out_folder.is_dir() && std::fs::create_dir(out_folder).is_err() {
            println!("{}", style("failed to create out folder").red().bold());
        }
    } else if let cli::Commands::List { disable_os_search } = args.command {
        init::list_template_names(disable_os_search);
    } else if let cli::Commands::Info { name, disable_os_search } = args.command {
        init::get_template_info(name, disable_os_search);
    } else if let cli::Commands::Build { debug } = args.command {
        builder::build(debug, std::env::current_dir().unwrap());
    } else if let cli::Commands::Count = args.command {
        builder::count(std::env::current_dir().unwrap());
    } else if let cli::Commands::Clean = args.command {
        builder::remove_files(std::env::current_dir().unwrap());
    } else if let cli::Commands::Get { name } = args.command {
        settings::get(name, std::env::current_dir().unwrap()).expect("An impossible error has just occurred");
    } else if let cli::Commands::Set { name, value } = args.command {
        settings::set(Some(name), Some(value), std::env::current_dir().unwrap());
    } else if let cli::Commands::New_Buildfile = args.command {
        init::create_build_file(std::env::current_dir().unwrap());
    } else {
        #[cfg(feature = "updater")]
        if let cli::Commands::Update { remove } = args.command {
            if remove {
                updater::remove_templates();
            } else {
                updater::download_templates(false);
            }
        }
    }
}
