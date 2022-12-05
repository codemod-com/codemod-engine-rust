use std::io::{BufReader, Read, Write};
use std::{fs::File, path::PathBuf};

use clap::Parser;
use command_line_arguments::CommandLineArguments;
use head_file::build_head_file_text;
use json::object;
use wax::{CandidatePath, Glob, Pattern};

mod command_line_arguments;
mod head;
mod head_file;
mod page_file;
mod paths;
mod tree;
mod queries;

use crate::page_file::build_page_file_text;
use crate::paths::{build_path_dto, get_apps_path_buf, build_byte_hash, build_output_path, get_pages_path_buf, build_page_document_path_bufs};
use crate::tree::build_tree;

fn build_path_bufs(directory: &String, pattern: &String, antipatterns: &Vec<Glob>) -> Vec<PathBuf> {
    let glob = Glob::new(&pattern).unwrap();

    return glob
        .walk(directory)
        .map(|walk_entry| walk_entry.unwrap())
        .map(|entry| {
            return entry.into_path();
        })
        .filter(|path| {
            let path = path.as_path();

            return !antipatterns
                .iter()
                .any(|ap| ap.is_match(CandidatePath::from(path)));
        })
        .collect::<Vec<PathBuf>>();
}

fn main() {
    let command_line_arguments = CommandLineArguments::parse();

    let antipatterns: Vec<Glob> = command_line_arguments
        .antipatterns
        .iter()
        .map(|p| Glob::new(p).unwrap())
        .collect();

    let page_path_bufs = build_path_bufs(
        &command_line_arguments.directory,
        &command_line_arguments.pattern,
        &antipatterns,
    );

    let language = tree_sitter_typescript::language_tsx();

    for old_path_buf in &page_path_bufs {
        let path_dto_option = build_path_dto(&command_line_arguments.output_directory_path, old_path_buf);

        if path_dto_option.is_none() {
            continue;
        }

        let path_dto = path_dto_option.unwrap();

        let old_file = File::open(&path_dto.old_path).unwrap();

        let mut reader = BufReader::new(old_file);
        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer).unwrap();

        let tree = build_tree(&language, &buffer);
        let root_node = tree.root_node();
        let bytes = buffer.as_ref();

        {
            let page_file_text = build_page_file_text(&language, &root_node, bytes);

            let mut file = File::create(&path_dto.page_output_path).unwrap();

            file.write_all(page_file_text).unwrap();

            let update = object! {
                k: 4,
                p: path_dto.new_page_path,
                o: path_dto.page_output_path,
                c: "nextjs"
            };

            println!("{}", json::stringify(update));
        }

        let head_file_text_option = build_head_file_text(&language, &root_node, bytes);

        if let Some(head_file_text) = head_file_text_option {
            let mut file = File::create(&path_dto.head_output_path).unwrap();

            file.write_all(head_file_text.as_bytes()).unwrap();

            let create_message = object! {
                k: 4,
                p: path_dto.new_head_path,
                o: path_dto.head_output_path,
                c: "nextjs"
            };

            println!("{}", json::stringify(create_message));
        }
    }

    // app/layout.tsx
    let pages_path_buf_option = page_path_bufs
        .first()
        .and_then(|path_buf| get_pages_path_buf(path_buf));

    if let Some(pages_path_buf) = pages_path_buf_option {
        let pp = build_page_document_path_bufs(pages_path_buf)
            .iter()
            .filter(|path_buf| path_buf.exists());

    //     let mut document_path_buf = app_path_buf.clone();
    //     document_path_buf.push("_document.tsx");

    //     let mut document_path_buf = app_path_buf.clone();
    //     document_path_buf.push("_app.tsx");


    //     app_path_buf.push("layout.tsx");

    //     let new_app_layout_path = app_path_buf.to_str().unwrap().to_string();

    //     let output_path = build_output_path(&command_line_arguments.output_directory_path, &new_app_layout_path, ".tsx");

    //     let create_message = object! {
    //         k: 4,
    //         p: new_app_layout_path,
    //         o: output_path,
    //         c: "nextjs"
    //     };

    //     println!("{}", json::stringify(create_message));
    }

    let finish_message = object! {
        k: 2,
    };

    println!("{}", json::stringify(finish_message));
}
