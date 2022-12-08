use clap::Parser;

#[derive(Debug, Parser)]
pub(crate) struct CompareCommandLineArguments {
    /// Pass the directory path
    #[clap(short = 'l', long)]
    pub(crate) left: String,

    /// Pass the glob pattern for file paths
    #[clap(short = 'r', long)]
    pub(crate) right: String,
}
