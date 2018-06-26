use html5ever::rcdom::{Handle, NodeData as Html5NodeData};

pub struct SimpleHtmlNode {
    pub data: NodeData,
    pub children: Vec<SimpleHtmlNode>,
}

pub enum NodeData {
    Element {
        name: String,
        attributes: Vec<(String, String)>,
    },
    Text(String),
    Unknown,
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

    pub fn from_html5ever_node(node: &Handle) -> SimpleHtmlNode {
        match node.data {
            Html5NodeData::Element {
                ref name,
                ref attrs,
                ..
            } => {
                let mut children = vec![];
                for child in node.children.borrow().iter() {
                    children.push(SimpleHtmlNode::from_html5ever_node(child));
                }
                let mut attributes = vec![];
                for attr in attrs.borrow().iter() {
                    let attribute = (attr.name.local.to_string(), attr.value.to_string());
                    attributes.push(attribute);
                }
                SimpleHtmlNode {
                    data: NodeData::Element {
                        name: name.local.to_string(),
                        attributes: attributes,
                    },
                    children: children,
                }
            }
            Html5NodeData::Text { ref contents, .. } => SimpleHtmlNode {
                data: NodeData::Text(contents.borrow().to_string()),
                children: vec![],
            },
            _ => SimpleHtmlNode {
                data: NodeData::Unknown,
                children: vec![],
            },
        }
    }
}
