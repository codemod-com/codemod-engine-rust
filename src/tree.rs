use tree_sitter::{Language, Parser, Tree};

pub fn build_tree(language: &Language, text: &impl AsRef<[u8]>) -> Tree {
    let mut parser = Parser::new();

    parser.set_language(*language).unwrap();

    return parser.parse(text, None).unwrap();
}
