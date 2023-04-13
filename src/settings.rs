use super::cli;
use super::exit;
use toml::{Deserializer, Serializer};

struct Settings {
    default_name: Option<String>,
}

pub fn set(setting: Option<String>, value: Option<String>) {
    todo!();
}

pub fn get(setting: Option<String>) {
    if setting.is_none() {
        list();
        return;
    }
    todo!();
}

fn list() {
    todo!();
}
