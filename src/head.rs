use std::ops::Range;

use tree_sitter::{Tree, Query, QueryCursor};

pub struct NextHeadImportStatement {
    identifier_range: Range<usize>,
}

pub fn find_next_head_import_statements(
    tree: &Tree,
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

    let query_matches = query_cursor.matches(&query, tree.root_node(), text_provider);

    return query_matches.flat_map(|query_match| {
        return query_match
            .nodes_for_capture_index(identifier_index)
            .map(|node| NextHeadImportStatement { identifier_range: node.byte_range() } )
            .collect::<Vec<NextHeadImportStatement>>();
    }).collect::<Vec<NextHeadImportStatement>>();
}