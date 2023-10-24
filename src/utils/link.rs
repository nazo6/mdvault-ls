use tree_sitter::Node;

use super::visit_node::{visit_node, ControlFlow, Step};

pub fn extract_link(node: &Node, src: &str) -> Vec<(String, Option<String>)> {
    let mut links = vec![];
    visit_node(node, |step| {
        let node = match step {
            Step::In(node) => node,
            Step::Out(node) => node,
        };
        if node.kind() == "inline_link" {
            let mut link = (None, None);
            for child in node.children(&mut node.walk()) {
                if child.kind() == "link_text" {
                    link.0 = Some(child.utf8_text(src.as_bytes()).unwrap().to_string());
                } else if child.kind() == "link_destination" {
                    link.1 = Some(child.utf8_text(src.as_bytes()).unwrap().to_string());
                }
            }

            if let (Some(text), dest) = link {
                links.push((text, dest))
            }
        }

        ControlFlow::Continue
    });

    links
}
