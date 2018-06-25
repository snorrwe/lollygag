// use std::mark::{Send, Sync};

pub struct SimpleHtmlNode {
    data: NodeData,
    children: Vec<SimpleHtmlNode>,
}

pub enum NodeData {
    Element {
        name: String,
        attributes: (String, String),
    },
    Text(String),
}

unsafe impl Send for SimpleHtmlNode {}
unsafe impl Sync for SimpleHtmlNode {}

impl SimpleHtmlNode {
    pub fn new(data: NodeData, children: Vec<SimpleHtmlNode>) -> SimpleHtmlNode {
        SimpleHtmlNode {
            data: data,
            children: children,
        }
    }
}
