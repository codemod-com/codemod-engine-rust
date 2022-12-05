use tree_sitter::Node;

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
