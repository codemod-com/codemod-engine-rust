use std::path::{PathBuf};

use clap::Parser;
use wax::{Glob, Pattern, CandidatePath};

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



fn main() {
    let command_line_arguments = CommandLineArguments::parse();

    let pattern = command_line_arguments.pattern;

    let antipatterns: Vec<Glob> = command_line_arguments.antipatterns
        .iter()
        .map(|p| Glob::new(p).unwrap())
        .collect();

    let glob = Glob::new(&pattern).unwrap();

    let path_bufs: Vec<PathBuf> = glob.walk(command_line_arguments.directory)
        .map(|walk_entry| walk_entry.unwrap())
        .map(|entry|  {
            return entry.into_path();
        })
        .filter(|path| {
            let path = path.as_path();

            return !antipatterns.iter().any(|ap| ap.is_match(CandidatePath::from(path)));
        })
        .collect();

    for path_buf in path_bufs {
        println!("{:?}", path_buf.display())
    }
}
