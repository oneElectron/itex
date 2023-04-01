use super::exit;

pub fn parse_clean_options(start: usize, args: Vec<String>) {
    let x = start;

    while x < args.len() {
        if args[x] == "--help" || args[x] == "-h" {
            print_clean_help();
            exit!(0);
        }
    }
}

pub fn print_clean_help() {
    println!("usage: itex clean <options>");
}
