use std::{option::Option, path::PathBuf, process::exit};

pub struct Options {
    pub template_name: String,
    pub search_path: Option<PathBuf>,
    pub print_help: bool,
    pub list_templates: bool,
    pub disable_os_search: bool,
    pub update: bool,
    pub info: bool,
}

pub fn parse_options(args: Vec<String>) -> Options {
    if args.len() <= 1 {
        println!("not enough arguments");
        print_help();
        exit(0);
    }

    let mut out = Options {
        template_name: "".to_string(),
        search_path: None,
        print_help: false,
        list_templates: false,
        disable_os_search: false,
        update: false,
        info: false,
    };
    let mut template_name: Option<String> = None;

    let mut x: usize = 1;
    while x < args.len() {
        if args[x] == "--help" || args[x] == "-h" || args[x].starts_with('?') || args[x] == "-help"
        {
            print_help();
            std::process::exit(0);
        }
        if args[x] == "--list" || args[x] == "-l" {
            out.list_templates = true;
        }
        if args[x] == "--disable-os-search" || args[x] == "-s" {
            out.disable_os_search = true;
        }
        if args[x] == "--update" || args[x] == "-u" {
            out.update = true;
        }
        if args[x] == "--info" || args[x] == "-i" {
            out.info = true;
        }
        if args[x] == "--search-path" || args[x] == "-p" {
            x += 1;
            if args.len() <= x {
                print_help();
                std::process::exit(0);
            }
            if args[x].starts_with('-') {
                print_help();
                exit(0);
            }

            out.search_path = Some(PathBuf::from(args[x].clone()));
        }

        if !args[x].starts_with('-') {
            template_name = Some(args[x].clone())
        }
        x += 1
    }

    if template_name.is_none() && (out.list_templates | out.update) {
        return out;
    }

    if template_name.is_none() {
        println!("Could not find template name");
        print_help();
        std::process::exit(0);
    }
    out.template_name = template_name.unwrap();

    out
}

#[cfg(feature = "updater")]
pub fn print_help() {
    println!("usage: itex <options> template");
    println!("  -l --list                 output a list of templates");
    println!(
        "  -s --disable-os-search    prevent itex from searching the os for the templates folder"
    );
    println!("  -u --update               update the itex-templates folder");
    println!("  -i --info                 get template info");
    // println!("  -p --search-path <path>   pass a templates directory");
    // println!("  -e --list-error-codes     list of return error codes, useful in scripts");
}

#[cfg(not(feature = "updater"))]
pub fn print_help() {
    println!("usage: itex <options> template");
    println!("  -l --list                 output a list of templates");
    println!(
        "  -s --disable-os-search    prevent itex from searching the os for the templates folder"
    );
    println!("  -i --info                 get template info");
}
