use tree_sitter::{Parser, Tree};

pub fn build_tree(
    text: &impl AsRef<[u8]>
) -> Tree {
    let mut parser = Parser::new();

    parser.set_language(tree_sitter_typescript::language_tsx()).unwrap();

    return parser.parse(text, None).unwrap();
}
