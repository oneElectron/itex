pub(crate) use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub(crate) struct Cli {
    // Command Line Options structure
    #[command(subcommand)]
    pub(crate) command: Commands,

    /// Path to project
    #[arg(short, long)]
    pub(crate) path: PathBuf,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// Build ITex project (requires an itex-build.toml file, and pdflatex to be installed)
    Build {
        /// Do not remove auxiliary build files (for debugging)
        #[arg(short, long)]
        debug: bool,

        /// Whether to build with draft mode enabled
        #[arg(long)]
        draft: bool,
    },

    /// Build ITex project in safe mode
    #[allow(non_camel_case_types)]
    Safe_Build,
    /// Count the number of words in the current ITex project (requires texcount to be installed)
    Count,
    /// Clean auxillary build files
    Clean,
    /// Initialize LaTex project
    Init {
        name: String,

        /// Disable looking in the os for itex-templates, only looks in . and ..
        #[arg(long)]
        disable_os_search: bool,

        /// The path to itex-templates
        #[arg(long)]
        search_path: Option<PathBuf>,
    },
    /// Get info about a template
    Info {
        /// The name of the template
        name: String,

        /// Disable searching the OS for the itex-templates folder
        #[arg(long)]
        disable_os_search: bool,

        /// The path to itex-templates
        #[arg(long)]
        search_path: Option<PathBuf>,
    },
    /// Get current value of a setting
    Get {
        /// The name of the option to get
        name: Option<String>,
    },
    /// List installed templates
    List {
        /// Disable searching the OS for the itex-templates folder
        #[arg(long)]
        disable_os_search: bool,

        /// The path to itex-templates
        #[arg(long)]
        search_path: Option<PathBuf>,
    },
    /// Create a new itex build file
    #[allow(non_camel_case_types)]
    New_Buildfile,
    /// Set a setting
    Set { name: String, value: String },

    #[cfg(feature = "updater")]
    /// Update installed templates
    Update {
        /// remove itex-templates folder
        #[arg(long, short)]
        remove: bool,
    },
}
