use tree_sitter::{Language, Node, Query, QueryCursor};

pub fn find_import_statements<'a>(
    language: &Language,
    node: &Node<'a>,
    text: &[u8],
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

    let source = source.replace("@_name", identifier.utf8_text(text).unwrap());

    let query = Query::new(*language, &source).unwrap();

    let import_statement_index = query.capture_index_for_name("import_statement").unwrap();

    let mut query_cursor = QueryCursor::new();

    let query_matches = query_cursor.matches(&query, *node, text);

    let nodes = query_matches
        .flat_map(|query_match| {
            return query_match
                .nodes_for_capture_index(import_statement_index)
                .collect::<Vec<Node>>();
        })
        .collect::<Vec<Node>>();

    nodes
}

pub fn build_head_text(head_node: &Node, import_statements: &Vec<Node>, source: &[u8]) -> String {
    let mut string: String = String::new();

    for import_statement in import_statements {
        let text = import_statement.utf8_text(source).unwrap();

        string.push_str(text);
        string.push('\n');
    }

    string.push_str("export default async function Head() {\n");
    string.push_str("return (<>\n");

    let child_count = head_node.child_count();

    for i in 0..child_count {
        if i == 0 || i == (child_count - 1) {
            continue;
        }

        let child_node = head_node.child(i).unwrap();
        let text = child_node.utf8_text(source).unwrap();

        string.push_str(text);
    }

    string.push_str("</>);\n");
    string.push_str("}\n");

    string
}
