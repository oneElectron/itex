use super::exit;

pub struct ListOptions {
    pub disable_os_search: bool,
}

pub fn parse_list_options(start: usize, args: Vec<String>) -> ListOptions {
    let mut x: usize = start;

    let mut disable_os_search: bool = false;
    while x < args.len() {
        if args[x] == *"--disable-os-search".to_string() {
            disable_os_search = true;
        }
        if args[x] == "--help" || args[x] == "-h" {
            print_list_help();
            exit!(0);
        }

        x += 1;
    }

    ListOptions { disable_os_search }
}

pub fn print_list_help() {
    println!("usage: itex list <options>");
}
