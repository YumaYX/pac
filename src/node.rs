#[derive(Debug)]
pub struct TextNode {
    pub raw: String,
    pub children: Vec<TextNode>,
}

impl TextNode {
    pub fn new(raw: String) -> Self {
        Self {
            raw,
            children: Vec::new(),
        }
    }
}
