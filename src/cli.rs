pub use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CLI {
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
    /// Set a setting
    Set(SetOptions),
}

#[derive(Args, Debug)]
pub struct BuildOptions {
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

    #[arg(long, help = "Disable looking in the os for itex-templates, only looks in . and ..")]
    pub disable_os_search: bool,

    #[arg(long, help = "The path to output to")]
    pub output_path: Option<String>,

    #[arg(long, help = "The path to itex-templates")]
    pub search_path: Option<String>,
}

#[derive(Args, Debug)]
pub struct InfoOptions {
    pub name: Option<String>,

    #[arg(long)]
    pub disable_os_search: bool,
}

#[derive(Args, Debug)]
pub struct ListOptions {
    #[arg(long)]
    pub disable_os_search: bool,
}

#[derive(Args, Debug)]
pub struct SetOptions {
    pub name: Option<String>,
    pub value: Option<String>,
}
