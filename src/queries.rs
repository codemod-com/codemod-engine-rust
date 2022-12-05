use tree_sitter::{Language, Node, Query, QueryCursor};

fn match_nodes<'a>(
    query: &Query,
    root_node: &Node<'a>,
    bytes: &'a [u8],
    capture_index: u32,
) -> Vec<Node<'a>> {
    let mut query_cursor = QueryCursor::new();

    let query_matches = query_cursor.matches(&query, *root_node, bytes);

    return query_matches
        .flat_map(|query_match| {
            return query_match
                .nodes_for_capture_index(capture_index)
                .collect::<Vec<Node>>();
        })
        .collect::<Vec<Node<'a>>>();
}

pub fn find_next_head_import_statements<'a>(
    language: &Language,
    root_node: &Node<'a>,
    bytes: &'a [u8],
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

    let capture_index = query.capture_index_for_name("identifier").unwrap();

    return match_nodes(&query, root_node, bytes, capture_index);
}

pub fn find_head_jsx_elements<'a>(
    language: &Language,
    root_node: &Node<'a>,
    bytes: &'a [u8],
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

    let source = source.replace("@_name", &statement.utf8_text(bytes).unwrap());

    let query = Query::new(*language, &source).unwrap();
    let capture_index = query.capture_index_for_name("jsx_element").unwrap();

    return match_nodes(&query, root_node, bytes, capture_index);
}

pub fn find_identifiers<'a>(
    language: &Language,
    root_node: &Node<'a>,
    bytes: &'a [u8],
) -> Vec<Node<'a>> {
    let query = Query::new(*language, r#"((identifier)* @identifier)"#).unwrap();
    let capture_index = query.capture_index_for_name("identifier").unwrap();

    return match_nodes(&query, root_node, bytes, capture_index);
}

pub fn find_import_statements<'a>(
    language: &Language,
    root_node: &Node<'a>,
    bytes: &'a [u8],
    identifier: &Node,
) -> Vec<Node<'a>> {
    let source = r#"(
        (import_statement
            (import_clause
                (named_imports
                    (import_specifier
                        name: (identifier) @name
                        (#eq? @name "@_name")
                    )
                )
            )
        )* @import_statement
    )"#;

    let source = source.replace("@_name", identifier.utf8_text(bytes).unwrap());

    let query = Query::new(*language, &source).unwrap();

    let capture_index = query.capture_index_for_name("import_statement").unwrap();

    return match_nodes(&query, root_node, bytes, capture_index);
}

pub fn find_jsx_self_closing_element<'a>(
    language: &Language,
    root_node: &Node<'a>,
    bytes: &'a [u8],
    identifier: &str,
) -> Vec<Node<'a>> {
    let source = r#"(
        (jsx_self_closing_element
            name: (identifier) @identifier
             (#eq? @identifier "@_identifier")
        ) @jsx_self_closing_element
    )"#;

    let source = source.replace("@_identifier", identifier);

    let query = Query::new(*language, &source).unwrap();

    let capture_index = query.capture_index_for_name("jsx_self_closing_element").unwrap();

    return match_nodes(&query, root_node, bytes, capture_index);
}