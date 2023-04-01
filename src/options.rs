use super::exit;
use super::parser::{Options, OptionsCommand};

pub fn handle_options(options_command: OptionsCommand) {
    if let OptionsCommand::Set(setting) = options_command {
        set(setting);
    } else if let OptionsCommand::Get(setting) = options_command {
        show(setting);
    } else if let OptionsCommand::List = options_command {
        list();
    }
}

fn set(setting: Options) {
    todo!();
}

fn show(setting: Options) {
    todo!();
}

fn list() {
    todo!();
}
