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

    if let cli::Commands::Init(options) = args.command {
        let output_path = match options.output_path {
            None => std::env::current_dir().expect("could not find pwd"),
            Some(p) => PathBuf::from(p),
        };

        if options.name.is_none() {
            println!("{}", style("No template name has been given").red().bold());
            exit!(0);
        }
        copy_template(options.name.unwrap().replace('\n', ""), output_path, options.disable_os_search);

        let out_folder = env::current_dir();
        if out_folder.is_err() {
            println!("{}", style("Could not find current path").red().bold());
        }

        let mut out_folder = out_folder.unwrap();
        out_folder.push("out");
        if !out_folder.is_dir() && std::fs::create_dir(out_folder).is_err() {
            println!("{}", style("failed to create out folder").red().bold());
        }
    } else if let cli::Commands::List(options) = args.command {
        init::list_template_names(options.disable_os_search);
    } else if let cli::Commands::Info(options) = args.command {
        if options.name.is_none() {
            println!("{}", style("No template name has been given").red().bold());
        }
        init::get_template_info(options.name.unwrap(), options.disable_os_search);
    } else if let cli::Commands::Build(options) = args.command {
        builder::build(options.debug, std::env::current_dir().unwrap());
    } else if let cli::Commands::Count = args.command {
        builder::count(std::env::current_dir().unwrap());
    } else if let cli::Commands::Clean = args.command {
        builder::remove_files();
    } else if let cli::Commands::Get(options) = args.command {
        settings::get(options.name, std::env::current_dir().unwrap()).expect("An impossible error has just occurred");
    } else if let cli::Commands::Set(options) = args.command {
        settings::set(options.name, options.value, std::env::current_dir().unwrap());
    } else if let cli::Commands::New_Buildfile = args.command {
        init::create_build_file(std::env::current_dir().unwrap());
    } else {
        #[cfg(feature = "updater")]
        if let cli::Commands::Update(_options) = args.command {
            updater::download_templates(false);
        }
    }
}
