use std::{path::PathBuf, fs::File, io::Write};

use json::object;
use tree_sitter::{Language, Node};

use crate::{paths::build_path_dto, read_file, tree::build_tree, head_file::build_head_file_text};

pub fn build_page_file_text<'bytes>(
    language: &Language,
    root_node: &Node,
    bytes: &'bytes [u8],
) -> &'bytes [u8] {
    bytes
}

pub fn build_page_directory_messages(
    output_directory_path: &String,
    language: &Language,
    old_path_buf: &PathBuf,
) {
    let path_dto_option = build_path_dto(&output_directory_path, &old_path_buf);

    if path_dto_option.is_none() {
        return;
    }

    let path_dto = path_dto_option.unwrap();

    let buffer = read_file(&path_dto.old_path);

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