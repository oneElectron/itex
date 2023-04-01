#![cfg(feature = "updater")]
use super::exit;

pub fn parse_updater_options(start: usize, args: Vec<String>) {
    let x = start;

    while x < args.len() {
        if args[x] == "--help" || args[x] == "-h" {
            print_updater_help();
            exit!(0);
        }
    }
}

pub fn print_updater_help() {
    println!("usage: itex update");
}
