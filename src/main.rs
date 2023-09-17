mod builder;
mod cli;
mod init;
mod macros;
mod path;
mod settings;
mod updater;

use cli::{Cli, Parser};
use console::style;
use init::copy_template;

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

            copy_template(name.replace('\n', ""), output_path.clone(), search_path, disable_os_search);

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
            init::get_template_info(name, search_path, disable_os_search);
        }

        cli::Commands::Build { debug, path } => {
            let og_path = std::env::current_dir().unwrap();
            std::env::set_current_dir(match path {
                Some(p) => p,
                None => path::find_itex_path().unwrap(),
            })
            .unwrap();

            builder::build(debug, std::env::current_dir().unwrap());

            std::env::set_current_dir(og_path).unwrap();
        }

        cli::Commands::Count => builder::count(std::env::current_dir().unwrap()),

        cli::Commands::Clean => builder::remove_files(std::env::current_dir().unwrap()),

        cli::Commands::Get { name } => {
            settings::get(name, std::env::current_dir().unwrap()).expect("An impossible error has just occurred");
        }

        cli::Commands::Set { name, value } => {
            settings::set(Some(name), Some(value), std::env::current_dir().unwrap());
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
