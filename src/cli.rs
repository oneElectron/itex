pub use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    // Command Line Options structure
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Build ITex project (requires an itex-build.toml file, and pdflatex to be installed)
    Build(BuildOptions),
    /// Count the number of words in the current ITex project (requires texcount to be installed)
    Count,
    /// Clean auxillary build files
    Clean,
    /// Initialize LaTex project
    Init(InitOptions),
    /// Get info about a template
    Info(InfoOptions),
    /// Get current value of a setting
    Get(GetOptions),
    /// List installed templates
    List(ListOptions),
    /// Create a new itex build file
    #[allow(non_camel_case_types)]
    New_Buildfile,
    /// Set a setting
    Set(SetOptions),

    #[cfg(feature = "updater")]
    /// Update installed templates
    Update(UpdaterOptions),
}

#[derive(Args, Debug)]
pub struct BuildOptions {
    /// Do not remove auxiliary build files (for debugging)
    #[arg(short, long)]
    pub debug: bool,
}

#[derive(Args, Debug)]
pub struct GetOptions {
    pub name: Option<String>,
}

#[derive(Args, Debug)]
pub struct InitOptions {
    pub name: Option<String>,

    /// Disable looking in the os for itex-templates, only looks in . and ..
    #[arg(long)]
    pub disable_os_search: bool,

    /// The path to output to
    #[arg(long)]
    pub output_path: Option<String>,

    /// The path to itex-templates
    #[arg(long)]
    pub search_path: Option<String>,
}

#[derive(Args, Debug)]
pub struct InfoOptions {
    /// The name of the template
    pub name: Option<String>,

    /// Disable searching the OS for the itex-templates folder
    #[arg(long)]
    pub disable_os_search: bool,
}

#[derive(Args, Debug)]
pub struct ListOptions {
    /// Disable searching the OS for the itex-templates folder
    #[arg(long)]
    pub disable_os_search: bool,
}

#[cfg(feature = "updater")]
#[derive(Args, Debug)]
pub struct UpdaterOptions {}

#[derive(Args, Debug)]
pub struct SetOptions {
    pub name: Option<String>,
    pub value: Option<String>,
}
