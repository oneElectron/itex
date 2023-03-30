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
    None,
    #[cfg(feature = "updater")]
    Update,
}

struct BuildOptions {
    debug: bool,
}

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
    #[cfg(debug_assertions)]
    println!("{} args: {:?}", style("[DEBUG - parse_options]").green(), args);

    if args.len() <= 1 {
        println!("{}", style("Not enough arguments").red().bold());
        print_help();
        exit!(0);
    }

    let mut output: Command = Command::None;
    let mut x = 1;
    /*
    while x < args.len() {
        // SOMETHING SOMETHING
        if args[x] == "--help" || args[x] == "-h" || args[x].starts_with('?') || args[x] == "-help"
        {
            print_help();
            exit(0);
        }
        x += 1;
    }
    x =  x - 2;
    */
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

    if let Command::Init(_, _, _, _) = output {
        let template_name = parse_template_name(x + 1, args.clone());
        if template_name.is_err() {
            println!("{}", style("No template name has been supplied").red().bold());
            print_help();
            exit!(0);
        }

        let init_options = parse_init_options(x + 1, args);

        let template_name = template_name.unwrap();
        output = Command::Init(
            template_name,
            init_options.search_path,
            init_options.output_path,
            init_options.disable_os_search,
        );
    } else if let Command::Build(_) = output {
        let build_options = parse_build_options(x + 1, args);

        output = Command::Build(build_options.debug);
    } else if let Command::List(_) = output {
        let list_options = parse_list_options(x + 1, args);

        output = Command::List(list_options.disable_os_search);
    } else if let Command::Info(_, _) = output {
        let template_name = parse_template_name(x + 1, args.clone());
        if template_name.is_err() {
            println!("{}", style("No template name has been supplied").red().bold());
            print_help();
            exit!(0);
        }
        let template_name = template_name.unwrap();

        let info_options = parse_info_options(x + 1, args);

        output = Command::Info(template_name, info_options.disable_os_search);
    } else if output == Command::None {
        println!("{}", style("No command given").red().bold());
        print_help();
        exit!(0);
    }

    output
}

#[rustfmt::skip]
pub fn print_help() {
    println!("usage: itex <command> <options>");
    println!("commands:");
    println!("  b  build            Build the project in the current folder (requires pdflatex");
    println!("     count            Count words in main.tex (requires texcount");
    println!("     clean            Clean the out directory");
    println!("  i  init             Copy a template into the current folder");
    println!("     info             Get template info");
    #[cfg(feature = "updater")]
    println!("     update           Update the templates folder");
    println!("  l  list             List installed templates");
    println!("options:");
    println!("  -h --help           Shows help menu for given command");
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

fn parse_build_options(start: usize, args: Vec<String>) -> BuildOptions {
    let mut x = start;
    let mut debug: bool = false;

    while x < args.len() {
        if args[x] == "--debug" || args[x] == "-d" {
            debug = true;
        }

        x += 1;
    }

    BuildOptions { debug }
}

fn parse_init_options(start: usize, args: Vec<String>) -> InitOptions {
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
        x += 1;
    }

    InitOptions {
        search_path,
        output_path,
        disable_os_search,
    }
}

fn parse_list_options(start: usize, args: Vec<String>) -> ListOptions {
    let mut x: usize = start;

    let mut disable_os_search: bool = false;
    while x < args.len() {
        if args[x] == *"--disable-os-search".to_string() {
            disable_os_search = true;
        }
        x += 1;
    }

    ListOptions { disable_os_search }
}

fn parse_info_options(start: usize, args: Vec<String>) -> InfoOptions {
    let mut x: usize = start;

    let mut disable_os_search: bool = false;
    while x < args.len() {
        if args[x] == *"--disable-os-search".to_string() {
            disable_os_search = true;
        }
        x += 1;
    }

    InfoOptions { disable_os_search }
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
}
