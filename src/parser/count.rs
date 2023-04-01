use super::exit;

pub fn parse_count_options(start: usize, args: Vec<String>) {
    let mut x = start;

    while x < args.len() {
        if args[x] == "--help" || args[x] == "-h" {
            print_count_help();
            exit!(0);
        }
        x += 1;
    }
}

pub fn print_count_help() {
    println!("usage: itex count <options>");
}
