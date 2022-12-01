use std::ops::Range;

use tree_sitter::{Query, QueryCursor, Node};

pub struct NextHeadImportStatement {
    pub identifier_range: Range<usize>,
    pub identifier_text: String,
}

pub fn find_next_head_import_statements(
    root_node: &Node,
    text_provider: &[u8],
) -> Vec<NextHeadImportStatement> {
    let query = Query::new(
        tree_sitter_typescript::language_typescript(),
        r#"(
            (import_statement
                (import_clause 
                    (identifier) @identifier
                )
                source: (string) @source
                (#eq? @source "'next/head'") 
            )
        )"#,
      ).unwrap();

    let identifier_index = query.capture_index_for_name("identifier").unwrap();

    let mut query_cursor = QueryCursor::new();

    let query_matches = query_cursor.matches(&query, *root_node, text_provider);

    return query_matches.flat_map(|query_match| {
        return query_match
            .nodes_for_capture_index(identifier_index)
            .map(|node| NextHeadImportStatement {
                identifier_range: node.byte_range(),
                identifier_text: node.utf8_text(text_provider).unwrap().to_string(),
            } )
            .collect::<Vec<NextHeadImportStatement>>();
    }).collect::<Vec<NextHeadImportStatement>>();
}

pub fn find_head_jsx_element_children(
    root_node: &Node,
    text_provider: &[u8],
    statement: &NextHeadImportStatement,
) {
    let source = r#"(
        (jsx_element
            open_tag: (jsx_opening_element
            	name: (identifier) @name
                (#eq? @name "@_name")
            )
        ) @jsx_element
    )"#;

    let source = source.replace("@_name", &statement.identifier_text);

    let query = Query::new(
        tree_sitter_typescript::language_typescript(),
        &source,
    ).unwrap();

    let jsx_element_index = query.capture_index_for_name("jsx_element").unwrap();

    let mut query_cursor = QueryCursor::new();

    let query_matches = query_cursor.matches(&query, *root_node, text_provider);

    for query_match in query_matches {

    }


}