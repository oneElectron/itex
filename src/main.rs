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

fn main() {
    if std::env::var_os("ITEX_DEBUG").is_some() {
        env_logger::init();
    }
    log::info!("Logging enabled");

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

            let name = name.replace('\n', "").trim().to_owned();

            log::trace!("name = {}", name);
            log::trace!("output_path = {:?}", output_path);
            log::trace!("search_path = {:?}", search_path);
            log::trace!("disable_os_search = {}", disable_os_search);

            init(name, output_path.clone(), search_path, disable_os_search);

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

            build(debug, draft);

            let e = std::env::set_current_dir(og_path);
            unwrap_result!(e, "Failed to set current directory back");
        }

        cli::Commands::Count { path } => {
            let og_path = path::change_to_itex_path(path);

            count();

            let e = std::env::set_current_dir(og_path);
            unwrap_result!(e, "Failed to set current directory back");
        }

        cli::Commands::Clean { path } => {
            let og_path = path::change_to_itex_path(path);

            clean(std::env::current_dir().unwrap(), &Settings::find_and_parse_toml());

            let e = std::env::set_current_dir(og_path);
            unwrap_result!(e, "Failed to set current directory back");
        }

        cli::Commands::Get { name, path } => {
            let og_path = path::change_to_itex_path(path);

            get(name).expect("An impossible error has just occurred");

            let e = std::env::set_current_dir(og_path);
            unwrap_result!(e, "Failed to set current directory back");
        }

        cli::Commands::Set { name, value, path } => {
            let og_path = path::change_to_itex_path(path);

            set(Some(name), Some(value));

            let e = std::env::set_current_dir(og_path);
            unwrap_result!(e, "Failed to set current directory back");
        }

        cli::Commands::New_Buildfile { path } => {
            let og_path = unwrap_result!(std::env::current_dir(), "Failed to get current directory");
            if path.is_some() {
                let e = std::env::set_current_dir(path.as_ref().unwrap());
                unwrap_result!(e, "Failed to change directory");
            }

            init::create_build_file(std::env::current_dir().unwrap());

            if path.is_some() {
                unwrap_result!(std::env::set_current_dir(&og_path), "Failed to change directory back");
            }
        }

        #[cfg(feature = "updater")]
        cli::Commands::Update { remove } => {
            if remove {
                updater::remove_templates();
            } else {
                updater::download_templates(false, false);
            }
        }
    }
}
