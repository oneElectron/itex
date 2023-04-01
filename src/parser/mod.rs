mod build;
mod clean;
mod count;
mod info;
mod init;
mod list;
mod options;
#[cfg(feature = "updater")]
mod updater;

use build::parse_build_options;
use clean::parse_clean_options;
use count::parse_count_options;
use info::parse_info_options;
use init::parse_init_options;
use list::parse_list_options;
use options::parse_options_options;
#[cfg(feature = "updater")]
use updater::parse_updater_options;

pub use build::print_build_help;
pub use clean::print_clean_help;
pub use count::print_count_help;
pub use info::print_info_help;
pub use init::print_init_help;
pub use list::print_list_help;
pub use options::print_options_help;
#[cfg(feature = "updater")]
pub use updater::print_updater_help;

pub use options::{Options, OptionsCommand};

use super::exit;
use console::style;
use std::{option::Option, path::PathBuf};

#[derive(std::fmt::Debug, PartialEq)]
pub enum Command {
    Build(bool),
    Count,
    Clean,
    Init(
        /* template_name */ String,
        /* search_path */ Option<PathBuf>, // TODO: Implement search_path
        /* output_path */ Option<PathBuf>,
        /* disable_os_search */ bool,
    ),
    Info(String, bool),
    List(bool),
    Options(OptionsCommand),
    None,
    Update,
}

pub fn parse_options(args: Vec<String>) -> Command {
    #[cfg(debug_assertions)]
    println!("{} args: {:?}", style("[DEBUG - parse_options]").green(), args);

    if args.len() <= 1 {
        println!("{}", style("Not enough arguments").red().bold());
        print_help();
        exit!(0);
    }

    let mut output: Command = Command::None;
    let mut x = 1;
    loop {
        if x >= args.len() {
            break;
        }
        #[cfg(feature = "updater")]
        {
            output = match args[x].as_str() {
                "update" => Command::Update,
                _ => Command::None,
            }
        }
        output = match args[x].as_str() {
            "init" | "i" => Command::Init("".to_string(), None, None, false),
            "count" => Command::Count,
            "clean" => Command::Clean,
            "build" | "b" => Command::Build(false),
            "list" | "l" => Command::List(false),
            "options" | "o" => Command::Options(OptionsCommand::None),
            "info" => Command::Info("".to_string(), false),
            "--help" | "-h" | "?" | "--Help" | "-H" => {
                print_help();
                exit!(0);
            }
            "updater" => break,
            _ => Command::None,
        };

        if output != Command::None {
            break;
        }

        x += 1;
    }

    if let Command::Build(_) = output {
        let build_options = parse_build_options(x + 1, args);
        output = Command::Build(build_options.debug);
    } else if let Command::Clean = output {
        let _clean_options = parse_clean_options(x + 1, args);
        output = Command::Clean;
    } else if let Command::Count = output {
        parse_count_options(x + 1, args);
    } else if let Command::Info(_, _) = output {
        let info_options = parse_info_options(x + 1, args.clone());

        let template_name = parse_template_name(x + 1, args);
        if template_name.is_err() {
            println!("{}", style("No template name has been supplied").red().bold());
            print_help();
            exit!(0);
        }
        let template_name = template_name.unwrap();

        output = Command::Info(template_name, info_options.disable_os_search);
    } else if let Command::Init(_, _, _, _) = output {
        let init_options = parse_init_options(x + 1, args.clone());

        let template_name = parse_template_name(x + 1, args);
        if template_name.is_err() {
            println!("{}", style("No template name has been supplied").red().bold());
            print_help();
            exit!(0);
        }

        let template_name = template_name.unwrap();
        output = Command::Init(
            template_name,
            init_options.search_path,
            init_options.output_path,
            init_options.disable_os_search,
        );
    } else if let Command::List(_) = output {
        let list_options = parse_list_options(x + 1, args);
        output = Command::List(list_options.disable_os_search);
    } else if let Command::Options(_) = output {
        let options = parse_options_options(x + 1, args);

        output = Command::Options(options);
    } else if output == Command::None {
        println!("{}", style("No command given").red().bold());
        print_help();
        exit!(0);
    } else if let Command::Update = output {
        #[cfg(feature = "updater")]
        let _options = parse_updater_options(x + 1, args);
        #[cfg(not(feature = "updater"))]
        {
            println!(
                "{}",
                style("This version of ITex was not built with the updater enabled").red().bold()
            )
        }

        output = Command::Update;
    }

    output
}

#[rustfmt::skip]
pub fn print_help() {
    println!("usage: itex <command> <options>");
    println!("commands:");
    println!("  b  build                    Build the project in the current folder (requires pdflatex");
    println!("     count                    Count words in main.tex (requires texcount");
    println!("     clean                    Clean the out directory");
    println!("  i  init                     Copy a template into the current folder");
    println!("     info                     Get template info");
    println!("  o  options                  Manage project options");
    #[cfg(feature = "updater")]
    println!("     update                   Update the templates folder");
    println!("  l  list                     List installed templates");
    println!("options:");
    println!("  -h --help                   Shows help menu for given command");
    // println!("  -p --search-path <path>   pass a templates directory");
    // println!("  -e --list-error-codes     list of return error codes, useful in scripts");
}

fn parse_template_name(start: usize, args: Vec<String>) -> Result<String, isize> {
    let mut x = start;
    while x < args.len() {
        if !args[x].starts_with('-') {
            return Ok(args[x].clone());
        }
        x += 1
    }

    Err(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::current_dir;

    #[test]
    fn parse_command_build() {
        let options = vec![
            "/opt/homebrew/bin/itex".to_string(),
            "build".to_string(),
            "default".to_string(),
            "--debug".to_string(),
        ];
        let output = parse_options(options);

        assert_eq!(output, Command::Build(true));
    }

    #[test]
    fn parse_command_init() {
        let options = vec!["/opt/homebrew/bin/itex".to_string(), "init".to_string(), "default".to_string()];
        let output = parse_options(options);

        if let Command::Init(template_name, _, _, _) = output {
            assert_eq!(template_name, "default".to_string());
        } else {
            panic!();
        }
    }

    #[test]
    fn parse_command_info() {
        let options = vec!["/opt/homebrew/bin/itex".to_string(), "info".to_string(), "default".to_string()];
        let output = parse_options(options);

        if let Command::Info(template_name, disable_os_search) = output {
            assert_eq!(template_name, "default".to_string());
            assert_eq!(disable_os_search, false);
        } else {
            panic!();
        }
    }

    #[test]
    fn parse_command_list() {
        let options = vec![
            "/opt/homebrew/bin/itex".to_string(),
            "list".to_string(),
            "--disable-os-search".to_string(),
        ];
        let output = parse_options(options);

        if let Command::List(disable_os_search) = output {
            assert_eq!(disable_os_search, true);
        } else {
            panic!();
        }
    }

    #[test]
    #[should_panic]
    fn parse_command_error() {
        let options = vec![
            "/opt/homebrew/bin/itex".to_string(),
            "HELLO".to_string(),
            "--disable-os_search".to_string(),
            "default".to_string(),
        ];
        let output = parse_options(options);

        assert_eq!(output, Command::None);
    }

    #[test]
    fn parse_init_options() {
        let search_output_path = current_dir().unwrap().to_str().unwrap().to_string();

        let options = vec![
            "/opt/homebrew/bin/itex".to_string(),
            "init".to_string(),
            "--search-path".to_string(),
            search_output_path.clone(),
            "default".to_string(),
            "--disable-os-search".to_string(),
            "--output-path".to_string(),
            search_output_path,
        ];
        let output = super::parse_init_options(2, options);

        assert_eq!(output.disable_os_search, true);
        assert_eq!(output.output_path, Some(current_dir().unwrap()));
        assert_eq!(output.search_path, Some(current_dir().unwrap()));
    }
    #[test]
    fn parse_option_options() {
        let args: Vec<String> = vec![
            "/opt/homebrew/itex".to_string(),
            "o".to_string(),
            "set".to_string(),
            "default_name".to_string(),
            "main".to_string(),
        ];

        let output = super::parse_options(args);
        if let Command::Options(option) = output {
            if let OptionsCommand::Set(option_name) = option {
                if let Options::default_name(value) = option_name {
                    assert_eq!(value, "main".to_string());
                } else {
                    panic!("failed to find value for options");
                }
            } else {
                panic!("failed to parse set command for options");
            }
        } else {
            panic!("failed to parse command for option");
        }
    }
}
