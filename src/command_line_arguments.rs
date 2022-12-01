use clap::Parser;

#[derive(Debug, Parser)]
pub(crate) struct CommandLineArguments {
    /// Pass the directory path
    #[clap(short = 'd', long)]
    pub(crate) directory: String,

    /// Pass the glob pattern for file paths
    #[clap(short = 'p', long)]
    pub(crate) pattern: String,

    /// Pass the glob antipattern for file paths
    #[clap(short = 'a', long)]
    pub(crate) antipatterns: Vec<String>,
    
    /// Pass the group(s) of codemods for execution
    #[clap(short = 'g', long)]
    pub(crate) group: Vec<String>,

    /// Pass the limit for the number of files to inspect
    #[clap(short = 'l', long)]
    pub(crate) limit: Option<u64>,

    /// Pass the limit for the number of files to inspect
    #[clap(short = 'o', long)]
    pub(crate) output_directory_path: Option<String>,
}
