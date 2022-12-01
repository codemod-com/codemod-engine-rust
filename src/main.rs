use std::fs::create_dir_all;
use std::io::{Read, Write};
use std::{path::PathBuf, fs::File};

use clap::Parser;
use json::object;
use wax::{Glob, Pattern, CandidatePath};

mod tree;
mod head;
mod paths;
use crate::head::{find_next_head_import_statements, find_head_jsx_elements, build_head_text};
use crate::paths::build_path_dto;
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

    for old_path_buf in page_path_bufs {
        let path_dto = build_path_dto(old_path_buf);

        let mut old_file = File::open(&path_dto.old_path).unwrap();

        create_dir_all(&path_dto.new_dir_path).unwrap();

        let mut new_file = File::create(&path_dto.new_page_path).unwrap();

        let mut buffer = String::new();

        old_file.read_to_string(&mut buffer).unwrap();

        new_file.write_all(buffer.as_bytes()).unwrap();

        let rewrite_message = object! {
            k: 3,
            i: path_dto.old_path,
            o: path_dto.new_page_path,
            c: "nextjs"
        };

        println!("{}", json::stringify(rewrite_message));

        let language = tree_sitter_typescript::language_tsx();

        let tree = build_tree(&language, &buffer);
        let root_node = tree.root_node();
        let text_provider = buffer.as_bytes();

        let statements = find_next_head_import_statements(&language, &root_node, text_provider);

        for statement in statements {
            let head_jsx_elements = find_head_jsx_elements(&language, &root_node, text_provider, &statement);

            for head_jsx_element in head_jsx_elements {
                let head_text = build_head_text(
                    &head_jsx_element,
                    text_provider,
                );

                println!("{}", head_text);
            }
                
        }
    }
}
