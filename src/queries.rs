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

pub fn find_head_jsx_elements<'a>(
    language: &Language,
    root_node: &Node<'a>,
    text_provider: &[u8],
    statement: &Node
) -> Vec<Node<'a>> {
    let source = r#"(
        (jsx_element
            open_tag: (jsx_opening_element
            	name: (identifier) @name
                (#eq? @name "@_name")
            )
        ) @jsx_element
    )"#;

    let source = source.replace("@_name", &statement.utf8_text(text_provider).unwrap());

    let query = Query::new(*language, &source).unwrap();

    let jsx_element_index = query.capture_index_for_name("jsx_element").unwrap();

    let mut query_cursor = QueryCursor::new();

    let query_matches = query_cursor.matches(&query, *root_node, text_provider);

    return query_matches
        .flat_map(|query_match| {
            return query_match
                .nodes_for_capture_index(jsx_element_index)
                .collect::<Vec<Node>>();
        })
        .collect::<Vec<Node>>();
}