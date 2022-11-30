use std::path::PathBuf;

use clap::Parser;
use glob::glob;
use glob::Pattern;

#[derive(Debug, Parser)]
pub(crate) struct CommandLineArguments {
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

fn main() {
    let command_line_arguments = CommandLineArguments::parse();

    let pattern = command_line_arguments.pattern;

    let antipatterns: Vec<Pattern> = command_line_arguments.antipatterns
        .iter()
        .map(|p| Pattern::new(p).unwrap())
        .collect();

    let path_bufs: Vec<PathBuf> = glob(&pattern).unwrap()
        .map(|p| p.unwrap())
        .filter(|path| {
            !antipatterns.iter().any(|ap| ap.matches_path(&path))
        })
        .collect();

    for path_buf in path_bufs {
        println!("{:?}", path_buf.display())
    }
}
