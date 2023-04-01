use super::{exit, print_help};
use console::style;
use std::path::PathBuf;

pub struct InitOptions {
    pub search_path: Option<PathBuf>,
    pub output_path: Option<PathBuf>,
    pub disable_os_search: bool,
}

pub fn parse_init_options(start: usize, args: Vec<String>) -> InitOptions {
    let mut x = start;
    let mut search_path: Option<PathBuf> = None;
    let mut output_path: Option<PathBuf> = None;
    let mut disable_os_search: bool = false;

    while x < args.len() {
        if args[x] == *"--search-path".to_string() {
            x += 1;
            if args[x].starts_with('-') {
                println!("{}", style("invalid search path").red().bold());
                print_help();
                exit!(0);
            }
            search_path = Some(PathBuf::from(args[x].clone()));
        }
        if args[x] == *"--output-path".to_string() {
            x += 1;
            if args[x].starts_with('-') {
                println!("{}", style("invalid output path").red().bold());
                print_help();
                exit!(0);
            }
            let tmp = PathBuf::from(args[x].clone());
            if !tmp.exists() {
                println!(
                    "{} \"{}\" {}",
                    style("Path").red().bold(),
                    tmp.as_os_str().to_str().unwrap(),
                    style("does not exist").red().bold()
                );
                exit!(0);
            }

            output_path = Some(PathBuf::from(args[x].clone()));
        }
        if args[x] == *"--disable-os-search".to_string() {
            disable_os_search = true;
        }
        if args[x] == "--help" || args[x] == "-h" {
            print_init_help();
            exit!(0);
        }

        x += 1;
    }

    InitOptions {
        search_path,
        output_path,
        disable_os_search,
    }
}

pub fn print_init_help() {
    println!("usage: itex init <options>");
    println!("     --search-path <path>     search in <path> for itex-templates");
    println!("     --output-path <path>     output template in <path>");
    println!("     --disable-os-search      disable searching the os for itex-templates");
}
