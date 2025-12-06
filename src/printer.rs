use crate::node::TextNode;

pub fn print_parent_children(nodes: &[TextNode], keyword: &str) {
    for parent in nodes {
        if parent.raw.contains(keyword) {
            println!("{}", parent.raw);
            for child in &parent.children {
                println!("{}", child.raw);
            }
        }
    }
}
