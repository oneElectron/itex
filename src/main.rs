mod build;
mod clean;
mod cli;
mod count;
mod info;
mod init;
mod macros;
mod path;
mod prelude;
mod programs;
mod resolve;
mod settings;
mod updater;

use cli::{Cli, Parser};
use console::style;

use prelude::*;

use log::trace;

fn main() {
    env_logger::init();
    let args = Cli::parse();

    match args.command {
        cli::Commands::Init {
            name,
            output_path,
            search_path,
            disable_os_search,
        } => {
            let mut output_path = match output_path {
                None => std::env::current_dir().expect("could not find pwd"),
                Some(p) => p,
            };
            trace!("output_path = {:?}", output_path);

            init(name.replace('\n', ""), output_path.clone(), search_path, disable_os_search);

            output_path.push("out");
            if !output_path.is_dir() && std::fs::create_dir(output_path).is_err() {
                println!("{}", style("failed to create out folder").red().bold());
            }
        }

        cli::Commands::List {
            disable_os_search,
            search_path,
        } => init::list_template_names(search_path, disable_os_search),

        cli::Commands::Info {
            name,
            disable_os_search,
            search_path,
        } => {
            let info = template_info(name, search_path, disable_os_search);

            println!("{}", info);
        }

        cli::Commands::Build { debug, draft, path } => {
            let og_path = path::change_to_itex_path(path);

            build(debug, draft, std::env::current_dir().unwrap());

            std::env::set_current_dir(og_path).unwrap();
        }

        cli::Commands::Count { path } => {
            let og_path = path::change_to_itex_path(path);

            count();

            std::env::set_current_dir(og_path).unwrap();
        }

        cli::Commands::Clean { path } => {
            let og_path = path::change_to_itex_path(path);

            clean(std::env::current_dir().unwrap(), Settings::find_and_parse_toml());

            std::env::set_current_dir(og_path).unwrap();
        }

        cli::Commands::Get { name, path } => {
            let og_path = path::change_to_itex_path(path);

            get(name).expect("An impossible error has just occurred");

            std::env::set_current_dir(og_path).unwrap();
        }

        cli::Commands::Set { name, value, path } => {
            let og_path = path::change_to_itex_path(path);

            set(Some(name), Some(value));

            std::env::set_current_dir(og_path).unwrap();
        }

        cli::Commands::New_Buildfile => init::create_build_file(std::env::current_dir().unwrap()),

        #[cfg(feature = "updater")]
        cli::Commands::Update { remove } => {
            if remove {
                updater::remove_templates();
            } else {
                updater::download_templates(false);
            }
        }
    }
}
