use std::io::{Read, Write, BufReader};
use std::{path::PathBuf, fs::File};

use clap::Parser;
use command_line_arguments::CommandLineArguments;
use head_file::build_head_file_text;
use json::object;
use wax::{Glob, Pattern, CandidatePath};

mod command_line_arguments;
mod tree;
mod head;
mod paths;
mod page_file;
mod head_file;

use crate::page_file::build_page_file_text;
use crate::paths::build_path_dto;
use crate::tree::build_tree;

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

    let language = tree_sitter_typescript::language_tsx();

    for old_path_buf in page_path_bufs {
        let path_dto = build_path_dto(old_path_buf);

        let old_file = File::open(&path_dto.old_path).unwrap();

        let mut reader = BufReader::new(old_file);
        let mut buffer = Vec::new();
    
        reader.read_to_end(&mut buffer).unwrap();

        let tree = build_tree(&language, &buffer);
        let root_node = tree.root_node();
        let bytes = buffer.as_ref();

        {
            let page_file_text = build_page_file_text(&language, &root_node, bytes);

            let mut file = File::create(&path_dto.new_page_path).unwrap();

            file.write_all(page_file_text).unwrap();

            let rewrite_message = object! {
                k: 3,
                i: path_dto.old_path,
                o: path_dto.new_page_path,
                c: "nextjs"
            };
    
            println!("{}", json::stringify(rewrite_message));
        }

        let head_file_text_option = build_head_file_text(&language, &root_node, bytes);

        if let Some(head_file_text) = head_file_text_option {
            let mut file = File::create(&path_dto.new_head_path).unwrap();

            file.write_all(head_file_text.as_bytes()).unwrap();
        }
    }
}
