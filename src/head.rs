use tree_sitter::{Tree, Query, QueryCursor};

pub fn find_head(
    tree: &Tree,
    text_provider: &[u8],
) {
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

    dbg!(query.capture_names());

    let mut query_cursor = QueryCursor::new();

    let query_matches = query_cursor.matches(&query, tree.root_node(), text_provider);

    for query_match in query_matches {
        dbg!(query_match.pattern_index);

        for capture in query_match.captures {
            dbg!(capture.index);
            let nbr = capture.node.byte_range();

            dbg!("{:#?}", nbr);
            let node_text = capture.node.utf8_text(text_provider).unwrap();

            dbg!(node_text);
            // let bytes = &text_provider[(psbr.end+1)..(nbr.start-1)];
            // let _rep = String::from_utf8_lossy(bytes);
        }
    }
}