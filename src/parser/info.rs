use super::exit;

pub struct InfoOptions {
    pub disable_os_search: bool,
}

pub fn parse_info_options(start: usize, args: Vec<String>) -> InfoOptions {
    let mut x: usize = start;

    let mut disable_os_search: bool = false;
    while x < args.len() {
        if args[x] == *"--disable-os-search".to_string() {
            disable_os_search = true;
        }
        if args[x] == "--help" || args[x] == "-h" {
            print_info_help();
            exit!(0);
        }

        x += 1;
    }

    InfoOptions { disable_os_search }
}

pub fn print_info_help() {
    println!("usage: itex info <options>")
}
