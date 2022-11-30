use std::{path::PathBuf, ffi::OsStr};

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

fn build_path_bufs(
    directory: &String,
    pattern: &String,
    antipatterns: &Vec<Glob>,
) -> Vec<PathBuf> {
    let glob = Glob::new(&pattern).unwrap();

    return glob.walk(directory)
        .map(|walk_entry| walk_entry.unwrap())
        .map(|entry|  {
            return entry.into_path();
        })
        .filter(|path| {
            let path = path.as_path();

            return !antipatterns.iter().any(|ap| ap.is_match(CandidatePath::from(path)));
        })
        .collect::<Vec<PathBuf>>();
}

fn main() {
    let command_line_arguments = CommandLineArguments::parse();

    let antipatterns: Vec<Glob> = command_line_arguments.antipatterns
        .iter()
        .map(|p| Glob::new(p).unwrap())
        .collect();

    let page_path_bufs = build_path_bufs(
        &command_line_arguments.directory,
        &String::from("**/pages/**/*.{ts,tsx}"),
        &antipatterns,
    );

    for path_buf in page_path_bufs {
        let extension = path_buf.extension().unwrap_or_default();
        let file_stem = path_buf.file_stem().unwrap_or_default();

        let mut new_path_buf: PathBuf = path_buf.into_iter().map(|osstr| {
            if osstr == "pages" {
                return OsStr::new("apps")
            }

            return osstr;
        }).collect();

        new_path_buf.pop();

        if file_stem != "index" {
            new_path_buf.push(file_stem);
        }

        let new_file_name = "page".to_owned() + extension.to_str().unwrap();

        new_path_buf.push(new_file_name);

        let new_path = path_buf.to_str().unwrap();

        println!("{:?}", new_path);
    }
}
