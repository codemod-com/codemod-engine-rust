use tree_sitter::Node;


pub fn build_page_file_text<'bytes>(
    root_node: &Node,
    bytes: &'bytes [u8],
) -> &'bytes [u8] {
    bytes
}