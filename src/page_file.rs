use tree_sitter::{Node, Language};

pub fn build_page_file_text<'bytes>(
    language: &Language,
    root_node: &Node,
    bytes: &'bytes [u8],
) -> &'bytes [u8] {
    bytes
}