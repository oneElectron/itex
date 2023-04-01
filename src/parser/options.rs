use super::exit;
use console::style;

#[derive(Debug, PartialEq)]
pub enum OptionsCommand {
    Set(Options),
    Get(Options),
    List,
    None,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, serde::Serialize)]
pub enum Options {
    default_name(String),
    tex_file(String),
    None,
}

pub fn parse_options_options(start: usize, args: Vec<String>) -> OptionsCommand {
    let mut x = start;
    let mut options_command = OptionsCommand::None;

    while x < args.len() {
        if args[x] == "--help" || args[x] == "-h" {
            print_options_help();
            exit!(0);
        }

        if !args[x].starts_with('-') {
            if (args[x] == "get" || args[x] == "set") && args.len() > x + 2 {
                let option = match args[x + 1].clone().as_str() {
                    // Add new options here
                    "default_name" => Options::default_name(args[x + 2].clone()),
                    "tex_file" => Options::tex_file(args[x + 2].clone()),
                    _ => Options::None,
                };

                options_command = match args[x].clone().as_str() {
                    "get" => OptionsCommand::Get(option),
                    "set" => {
                        if option != Options::None {
                            OptionsCommand::Set(option)
                        } else {
                            println!("{}", style("No value given for option").red().bold());
                            print_options_help();
                            exit!(0);
                        }
                    }
                    _ => panic!("Unreachable code reached!"),
                };
            } else if args[x] == "list" {
                options_command = OptionsCommand::List;
            }
        }
        x += 1;
    }

    if let OptionsCommand::None = options_command {
        println!("{}", style("No options command given").red().bold());
        print_options_help();
        exit!(0);
    }

    options_command
}

pub fn print_options_help() {
    println!("usage: itex set <command> <options>");
    println!("commands:");
    println!("    set <option> <value>      Set an option");
    println!("    get <option>              Get the value of an option");
    println!("    list                      List all available options");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_options_set_default_name() {
        let args: Vec<String> = vec![
            "itex".to_string(),
            "options".to_string(),
            "set".to_string(),
            "default_name".to_string(),
            "main".to_string(),
        ];

        let output = parse_options_options(2, args);

        if let OptionsCommand::Set(name) = output {
            assert_eq!(name, Options::default_name("main".to_string()));
        } else {
            panic!("Option returned was not default_name");
        }
    }

    #[test]
    fn parse_options_get_default_name() {
        let args: Vec<String> = vec![
            "itex".to_string(),
            "options".to_string(),
            "get".to_string(),
            "default_name".to_string(),
            "main".to_string(),
        ];

        let output = parse_options_options(2, args);

        if let OptionsCommand::Get(name) = output {
            assert_eq!(name, Options::default_name("main".to_string()));
        } else {
            panic!("Option returned was not default_name");
        }
    }

    #[test]
    fn parse_options_list() {
        let args: Vec<String> = vec!["itex".to_string(), "options".to_string(), "list".to_string()];

        let output = parse_options_options(2, args);

        if let OptionsCommand::List = output {
        } else {
            panic!("Option returned was not default_name");
        }
    }

    #[test]
    #[should_panic]
    fn parse_options_get_error() {
        let args: Vec<String> = vec!["itex".to_string(), "options".to_string(), "get".to_string()];

        let output = parse_options_options(2, args);

        if let OptionsCommand::Get(name) = output {
            assert_eq!(name, Options::default_name("main".to_string()));
        } else {
            panic!("Option returned was not default_name");
        }
    }
}
