mod global;
mod cli;

pub(crate) use global::Settings;
pub(crate) use global::get as get_global;
pub(crate) use global::set as set_global;

pub(crate) use cli::Cli;
pub(crate) use cli::Commands as Command;

use clap::Parser;

pub(crate) struct Config {
    pub(crate) global: global::Settings,
    pub(crate) cli: cli::Cli,
}

impl Config {
    pub(crate) fn parse() -> Self {
        if std::env::var_os("ITEX_DEBUG").is_some() || (cfg!(debug_assertions) && std::env::var_os("RUST_LOG").is_some()) {
            env_logger::init();
        }
        log::info!("Logging enabled");
        
        Self {
            cli: cli::Cli::parse(),
            global: global::Settings::parse(),
        }
    }
}
