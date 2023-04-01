use super::super::exit;

pub struct BuildOptions {
    pub debug: bool,
}

pub fn parse_build_options(start: usize, args: Vec<String>) -> BuildOptions {
    let mut x = start;
    let mut debug: bool = false;

    while x < args.len() {
        if args[x] == "--debug" || args[x] == "-d" {
            debug = true;
        }

        if args[x] == "--help" || args[x] == "-h" {
            print_build_help();
            exit!(0);
        }

        x += 1;
    }

    BuildOptions { debug }
}

pub fn print_build_help() {
    println!("usage: itex build <options>");
    println!("options:");
    println!("  -d --debug                  Debug mode. Does not remove extra files");
}
