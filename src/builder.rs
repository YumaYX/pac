use crate::node::TextNode;

pub fn build_tree_with_trigger(lines: &[String], trigger: &str) -> Vec<TextNode> {
    let mut roots: Vec<TextNode> = Vec::new();
    let mut current_parent: Option<TextNode> = None;

    for line in lines {
        if line.contains(trigger) {
            if let Some(parent) = current_parent.take() {
                roots.push(parent);
            }
            current_parent = Some(TextNode::new(line.clone()));
        } else {
            match current_parent.as_mut() {
                Some(parent) => parent.children.push(TextNode::new(line.clone())),
                None => roots.push(TextNode::new(line.clone())),
            }
        }
    }

    if let Some(parent) = current_parent {
        roots.push(parent);
    }

    roots
}
