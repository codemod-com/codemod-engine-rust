use std::fs::create_dir_all;
use std::io::{Read, Write};
use std::{path::PathBuf, ffi::OsStr, fs::File};

use clap::Parser;
use json::object;
use wax::{Glob, Pattern, CandidatePath};

mod tree;
mod head;
use crate::head::find_next_head_import_statements;
use crate::tree::build_tree;

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

    for out_path_buf in page_path_bufs {
        let extension = out_path_buf.extension().unwrap_or_default();
        let file_stem = out_path_buf.file_stem().unwrap_or_default();

        let mut new_path_buf: PathBuf = out_path_buf.into_iter().map(|osstr| {
            if osstr == "pages" {
                return OsStr::new("apps")
            }

            return osstr;
        }).collect();

        new_path_buf.pop();

        if file_stem != "index" {
            new_path_buf.push(file_stem);
        }

        let new_file_name = "page.".to_owned() + extension.to_str().unwrap();

        new_path_buf.push(new_file_name);

        let new_path = new_path_buf.to_str().unwrap();

        {
            let mut dir_path_buf = new_path_buf.clone();
            dir_path_buf.pop();

            create_dir_all(dir_path_buf).unwrap();
        }

        let old_path = out_path_buf.to_str().unwrap().clone();

        let mut old_file = File::open(old_path).unwrap();

        let mut new_file = File::create(new_path).unwrap();

        let mut buffer = String::new();

        old_file.read_to_string(&mut buffer).unwrap();

        new_file.write_all(buffer.as_bytes()).unwrap();

        let rewrite_message = object! {
            k: 3,
            i: old_path,
            o: new_path,
            c: "nextjs"
        };

        println!("{}", json::stringify(rewrite_message));

        let tree = build_tree(&buffer);

        find_next_head_import_statements(&tree, &buffer.as_bytes());
    }
}
