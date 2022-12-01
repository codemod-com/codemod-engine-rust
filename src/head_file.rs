use tree_sitter::Node;

use std::{collections::HashSet};

use tree_sitter::{Language};

use crate::head::{find_next_head_import_statements, find_head_jsx_elements, find_identifiers, find_import_statements, build_head_text};

pub fn build_head_file_text(
    language: &Language,
    root_node: &Node,
    bytes: &[u8],
) -> Option<String> {
    let statements = find_next_head_import_statements(&language, &root_node, bytes);

    for statement in statements {
        let head_jsx_elements = find_head_jsx_elements(&language, &root_node, bytes, &statement);

        for head_jsx_element in head_jsx_elements {
            let identifiers = find_identifiers(&language, &root_node, bytes);

            let import_statements = identifiers.iter()
                .flat_map(|identifier| find_import_statements(&language, &root_node, bytes, &identifier))
                .collect::<HashSet<Node>>()
                .iter()
                .cloned()
                .collect::<Vec<Node>>();

            let head_text = build_head_text(
                &head_jsx_element,
                &import_statements,
                bytes,
            );

            return Some(head_text);
        }
    }

    return None;
}
