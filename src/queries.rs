use tree_sitter::{Language, Node, Query, QueryCursor};

pub fn find_next_head_import_statements<'a>(
    language: &Language,
    root_node: &Node<'a>,
    bytes: &[u8],
) -> Vec<Node<'a>> {
    let query = Query::new(
        *language,
        r#"(
            (import_statement
                (import_clause 
                    (identifier) @identifier
                )
                source: (string) @source
                (#eq? @source "'next/head'") 
            )
        )"#,
    )
    .unwrap();

    let identifier_index = query.capture_index_for_name("identifier").unwrap();

    let mut query_cursor = QueryCursor::new();

    let query_matches = query_cursor.matches(&query, *root_node, bytes);

    return query_matches
        .flat_map(|query_match| {
            return query_match
                .nodes_for_capture_index(identifier_index)
                .collect::<Vec<Node>>();
        })
        .collect::<Vec<Node>>();
}