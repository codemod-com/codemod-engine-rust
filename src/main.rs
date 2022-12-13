use std::ffi::OsStr;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::thread;
use std::{fs::File, path::PathBuf};

use clap::Parser;
use command_line_arguments::CommandLineArguments;
use json::{object, parse};
use wax::{CandidatePath, Glob, Pattern};

mod command_line_arguments;
mod compare_command_line_arguments;
mod head;
mod head_file;
mod page_file;
mod paths;
mod queries;
mod tree;

use crate::page_file::build_page_directory_messages;
use crate::paths::{build_output_path, build_page_document_path_buf_option, get_pages_path_buf};
use crate::queries::find_jsx_self_closing_element;
use crate::tree::build_tree;

use tree_sitter::Language;
use tree_sitter_traversal::{traverse_tree, Order};

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

fn get_node_texts(path: &String, language: &Language) -> Vec<String> {
    let left_buffer = read_file(path);
    let left_tree = build_tree(&language, &left_buffer);

    let mut left_iterator = traverse_tree(&left_tree, Order::Post)
        .into_iter()
        .filter(|node| node.child_count() == 0)
        .collect::<Vec<_>>();

    left_iterator.sort_by_key(|node| node.byte_range().start);

    let unimportant_strings = [String::from("("), String::from(")"), String::from(";")];

    return left_iterator
        .into_iter()
        .map(|node| node.utf8_text(&left_buffer).unwrap().to_string())
        .filter(|str| {
            !unimportant_strings.contains(str) && str.replace("\n", "").replace(" ", "") != ""
        })
        .collect::<Vec<_>>();
}

fn compare_string_vectors(left: &Vec<String>, right: &Vec<String>) -> bool {
    if left.len() != right.len() {
        return false;
    }

    for i in 0..left.len() {
        let left_text = &left[i];
        let right_text = &right[i];

        if left_text != right_text {
            return false;
        }
    }

    return true;
}

fn main() {
    let language = tree_sitter_typescript::language_tsx();

    let command_line_arguments = CommandLineArguments::try_parse();

    if command_line_arguments.is_err() {
        let stdin = std::io::stdin();

        for line_result in stdin.lock().lines() {
            let line = line_result.unwrap();
            let json_value = parse(&line).unwrap();

            let message_kind_option = match json_value.has_key("k") {
                true => json_value["k"].as_u8(),
                false => None,
            };

            let message_id_option = match json_value.has_key("i") {
                true => json_value["i"].as_str(),
                false => None,
            };

            let left_path_option = match json_value.has_key("l") {
                true => json_value["l"].as_str(),
                false => None,
            };

            let right_path_option = match json_value.has_key("r") {
                true => json_value["r"].as_str(),
                false => None,
            };

            match (
                message_kind_option,
                message_id_option,
                left_path_option,
                right_path_option,
            ) {
                (Some(message_kind), Some(message_id), Some(left_path), Some(right_path)) => {
                    let left_node_texts = get_node_texts(&left_path.to_string(), &language);
                    let right_node_texts = get_node_texts(&right_path.to_string(), &language);

                    let equal = compare_string_vectors(&left_node_texts, &right_node_texts);

                    let message = object! {
                        k: message_kind,
                        i: message_id,
                        e: equal,
                    };

                    println!("{}", json::stringify(message));
                }
                _ => {}
            };
        }
    }

    let command_line_arguments = command_line_arguments.unwrap();

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

    let pages_path_buf_option = &page_path_bufs
        .first()
        .and_then(|path_buf| get_pages_path_buf(path_buf));

    let mut handles = Vec::new();

    for old_path_buf in page_path_bufs {
        let output_directory_path = command_line_arguments.output_directory_path.clone();

        let handle = thread::spawn(move || {
            build_page_directory_messages(&output_directory_path, &language, &old_path_buf);
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

            let script_jsx_elements: Vec<&str> =
                find_jsx_self_closing_element(&language, &root_node, bytes, "script")
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
                "tsx",
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
