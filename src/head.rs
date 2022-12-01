use std::{ops::Range, collections::HashSet};

use tree_sitter::{Query, QueryCursor, Node, Language};

#[derive(Debug)]
pub struct NextHeadImportStatement {
    pub identifier_range: Range<usize>,
    pub identifier_text: String,
}

pub fn find_next_head_import_statements(
    language: &Language,
    root_node: &Node,
    text_provider: &[u8],
) -> Vec<NextHeadImportStatement> {
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
    language: &Language,
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
        tree_sitter_typescript::language_tsx(),
        &source,
    ).unwrap();

    let jsx_element_index = query.capture_index_for_name("jsx_element").unwrap();

    let mut query_cursor = QueryCursor::new();

    let query_matches = query_cursor.matches(&query, *root_node, text_provider);

    let mut child_nodes = Vec::<Node>::new();

    for query_match in query_matches {
        let nodes = query_match.nodes_for_capture_index(jsx_element_index);

        nodes.for_each(|node| {
            let child_count = node.child_count();

            for i in 0..child_count {
                if i == 0 || i == (child_count - 1) {
                    continue;
                }

                child_nodes.push(node.child(i).unwrap());
            }
        });
    }

    let head_text = build_head_text(&child_nodes, text_provider);

    println!("{}", head_text);

    for child_node in child_nodes {
        let identifiers = find_identifiers(
            language,
            &child_node,
            text_provider,
        );

        println!("{:#?}", identifiers);
    }
}

pub fn find_identifiers(
    language: &Language,
    node: &Node,
    text: &[u8],
) -> HashSet<String> {
    let query = Query::new(
        *language,
        r#"((identifier)* @identifier)"#,
    ).unwrap();

    let mut query_cursor = QueryCursor::new();

    let identifiers = query_cursor
        .captures(&query, *node, text)
        .flat_map(|m| m.0.captures)
        .map(|c| c.node.utf8_text(text).unwrap().to_string())
        .collect::<HashSet<String>>();

    identifiers
}

pub fn build_head_text(
    children: &Vec<Node>,
    source: &[u8],
) -> String {
    let mut string: String = String::new();

    string.push_str("export default async function Head() {\n");
    string.push_str("return (<>\n");

    for child in children {
        let text = child.utf8_text(source).unwrap();

        string.push_str(text);
    }

    string.push_str("<>);\n");
    string.push_str("}\n");

    string
}