use console::style;
use std::{option::Option, path::PathBuf, process::exit};

#[derive(std::fmt::Debug, PartialEq)]
pub enum Command {
    Build,
    Init(
        /* template_name */ String,
        /* search_path */ Option<PathBuf>, // TODO: Implement search_path
        /* output_path */ Option<PathBuf>,
        /* disable_os_search */ bool,
    ),
    Info(String, bool), // TODO: implement this
    List(bool),         // TODO: implement disable_os_search
    None,
    #[cfg(feature = "updater")]
    Update,
}

struct BuildOptions {}

struct InitOptions {
    search_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
    disable_os_search: bool,
}

struct ListOptions {
    disable_os_search: bool,
}

struct InfoOptions {
    disable_os_search: bool,
}

pub fn parse_options(args: Vec<String>) -> Command {
    if args.len() <= 1 {
        println!("Not enough arguments");
        print_help();
        exit(0);
    }

    let mut output: Command = Command::None;
    let mut x = 1;

    while x < args.len() {
        // SOMEHTING SOMETHING
        if args[x] == "--help" || args[x] == "-h" || args[x].starts_with('?') || args[x] == "-help"
        {
            print_help();
            exit(0);
        }
        x += 1;
    }

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
            "init" | "i" => Command::Init("PLACEHOLDER".to_string(), None, None, false),
            "build" | "b" => Command::Build,
            "list" | "l" => Command::List(false),
            "info" => Command::Info("PLACEHOLDER".to_string(), false),
            "--help" | "-h" | "?" | "--Help" | "-H" => {
                print_help();
                exit(0)
            }
            "updater" => break,
            _ => Command::None,
        };

        if Command::None != output {
            break;
        }

        x += 1;
    }

    if let Command::Init(_, _, _, _) = output {
        let template_name = parse_template_name(x + 1, args.clone());
        if template_name.is_err() {
            println!("No template name has been supplied");
            print_help();
            exit(1);
        }

        let init_options = parse_init_options(x + 1, args.clone());

        let template_name = template_name.unwrap();
        output = Command::Init(
            template_name,
            init_options.search_path,
            init_options.output_path,
            init_options.disable_os_search,
        );
    } else if let Command::Build = output {
        let build_options = parse_build_options();

        output = Command::Build;
    } else if let Command::List(_) = output {
        let list_options = parse_list_options(x + 1, args.clone());

        output = Command::List(list_options.disable_os_search);
    } else if let Command::Info(_, _) = output {
        let template_name = parse_template_name(x + 1, args.clone());
        if template_name.is_err() {
            println!("No template name has been supplied");
            print_help();
            exit(1);
        }
        let template_name = template_name.unwrap();

        let info_options = parse_info_options(x + 1, args.clone());

        output = Command::Info(template_name, info_options.disable_os_search);
    } else if output == Command::None {
        println!("No command given");
        print_help();
        exit(0);
    }

    println!("{:?}", output);

    output
}

#[rustfmt::skip]
pub fn print_help() {
    println!("usage: itex <command> <options>");
    println!("commands:");
    println!("  b  build                  build the project in the current folder");
    println!("  i  init                   Copy a template into the current folder");
    println!("     info                   Get template info");
    #[cfg(feature = "updater")]
    println!("     update                 Update the templates folder");
    println!("  l  list                   List installed templates");
    println!("options:");
    println!("  -h --help                 Shows help menu, use <command> --help to show help menu for specific commands");
    println!("  -o --output <path>        output template to given folder <path>");
    println!("  -s --disable-os-search    prevent itex from searching the os for the templates folder");
    // println!("  -p --search-path <path>   pass a templates directory");
    // println!("  -e --list-error-codes     list of return error codes, useful in scripts");
}

fn parse_template_name(start: usize, args: Vec<String>) -> Result<String, isize> {
    let mut x = start;
    while x < args.len() {
        if !args[x].starts_with("-") {
            return Ok(args[x].clone());
        }
        x += 1
    }

    Err(1)
}

fn parse_build_options() -> BuildOptions {
    return BuildOptions {};
}

fn parse_init_options(start: usize, args: Vec<String>) -> InitOptions {
    let mut x = start;
    let mut search_path: Option<PathBuf> = None;
    let mut output_path: Option<PathBuf> = None;
    let mut disable_os_search: bool = false;

    while x < args.len() {
        if args[x] == "--search-path".to_string() {
            x += 1;
            if args[x].starts_with("-") {
                println!("invalid search path");
                print_help();
            }
            search_path = Some(PathBuf::from(args[x].clone()));
        }
        if args[x] == "--output-path".to_string() {
            x += 1;
            if args[x].starts_with("-") {
                println!("{}", style("invalid output path").red().bold());
                print_help();
            }
            let tmp = PathBuf::from(args[x].clone());
            if !tmp.exists() {
                println!("{}", style("Path does not exist").red().bold());
            }

            output_path = Some(PathBuf::from(args[x].clone()));
        }
        if args[x] == "--disable-os-search".to_string() {
            disable_os_search = true;
        }
        x += 1;
    }

    return InitOptions {
        search_path,
        output_path,
        disable_os_search,
    };
}

fn parse_list_options(start: usize, args: Vec<String>) -> ListOptions {
    let mut x: usize = start;

    let mut disable_os_search: bool = false;
    while x < args.len() {
        if args[x] == "--disable-os-search".to_string() {
            disable_os_search = true;
        }
        x += 1;
    }

    return ListOptions { disable_os_search };
}

fn parse_info_options(start: usize, args: Vec<String>) -> InfoOptions {
    let mut x: usize = start;

    let mut disable_os_search: bool = false;
    while x < args.len() {
        if args[x] == "--disable-os-search".to_string() {
            disable_os_search = true;
        }
        x += 1;
    }

    return InfoOptions { disable_os_search };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command_build() {
        let options = vec![
            "/opt/homebrew/bin/itex".to_string(),
            "build".to_string(),
            "default".to_string(),
        ];
        let output = parse_options(options);

        assert_eq!(output, Command::Build);
    }

    #[test]
    fn parse_command_init() {
        let options = vec![
            "/opt/homebrew/bin/itex".to_string(),
            "init".to_string(),
            "default".to_string(),
        ];
        let output = parse_options(options);

        if let Command::Init(template_name, _, _, _) = output {
            assert_eq!(template_name, "default".to_string());
        } else {
            panic!();
        }
    }

    #[test]
    fn parse_command_info() {
        let options = vec![
            "/opt/homebrew/bin/itex".to_string(),
            "info".to_string(),
            "default".to_string(),
        ];
        let output = parse_options(options);

        if let Command::Init(template_name, _, _, _) = output {
            assert_eq!(template_name, "default".to_string());
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
        let options = vec![
            "/opt/homebrew/bin/itex".to_string(),
            "init".to_string(),
            "--search-path".to_string(),
            "~/Documents".to_string(),
            "default".to_string(),
            "--disable-os-search".to_string(),
            "--output-path".to_string(),
            "~/Documents".to_string(),
        ];
        let output = super::parse_init_options(2, options);

        assert_eq!(output.disable_os_search, true);
        assert_eq!(output.output_path, Some(PathBuf::from("~/Documents")));
        assert_eq!(output.search_path, Some(PathBuf::from("~/Documents")));
    }
}
