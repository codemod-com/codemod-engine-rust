use std::ffi::OsStr;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::thread;
use std::{fs::File, path::PathBuf};

use clap::Parser;
use command_line_arguments::CommandLineArguments;
use json::object;
use wax::{CandidatePath, Glob, Pattern};

mod command_line_arguments;
mod head;
mod head_file;
mod page_file;
mod paths;
mod tree;
mod queries;

use crate::page_file::build_page_directory_messages;
use crate::paths::{get_pages_path_buf, build_page_document_path_buf_option, build_output_path};
use crate::queries::find_jsx_self_closing_element;
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

fn read_file<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let file = File::open(path).unwrap();

    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer).unwrap();

    buffer
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

    let pages_path_buf_option = &page_path_bufs
        .first()
        .and_then(|path_buf| get_pages_path_buf(path_buf));

    let mut handles = Vec::new();

    for old_path_buf in page_path_bufs {
        let output_directory_path = command_line_arguments.output_directory_path.clone();

        let handle = thread::spawn(move || {
            build_page_directory_messages(
                &output_directory_path,
                &language,
                &old_path_buf,
            );
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // app/layout.tsx

    if let Some(pages_path_buf) = pages_path_buf_option {
        let page_document_path_buf_option = build_page_document_path_buf_option(pages_path_buf);

        if let Some(page_document_path_buf) = page_document_path_buf_option {
            let buffer = read_file(&page_document_path_buf);

            let tree = build_tree(&language, &buffer);
            let root_node = tree.root_node();
            let bytes: &[u8] = buffer.as_ref();

            let script_jsx_elements: Vec<&str> = find_jsx_self_closing_element(
                    &language,
                    &root_node,
                    bytes,
                    "script"
                )
                .iter()
                .map(|node| node.utf8_text(bytes).unwrap())
                .collect();

            let mut body = String::from("");

            for element in script_jsx_elements {
                body.push_str(element);
            }

            let mut app_path_buf: PathBuf = pages_path_buf
                .into_iter()
                .map(|osstr| {
                    if osstr == "pages" {
                        return OsStr::new("app");
                    }

                    return osstr;
                })
                .collect();

            app_path_buf.push("layout.tsx");

            let new_app_layout_path = app_path_buf.to_str().unwrap().to_string();

            let output_path = build_output_path(
                &command_line_arguments.output_directory_path,
                &new_app_layout_path,
                "tsx"
            );

            let mut file = File::create(&output_path).unwrap();

            file.write_all(body.as_bytes()).unwrap();

            let create_message = object! {
                k: 4,
                p: new_app_layout_path,
                o: output_path,
                c: "nextjs"
            };

            println!("{}", json::stringify(create_message));
        }
    }

    let finish_message = object! {
        k: 2,
    };

    println!("{}", json::stringify(finish_message));
}
