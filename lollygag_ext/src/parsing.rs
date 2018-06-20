use html5ever::parse_document;
use html5ever::rcdom::{Handle, NodeData};

pub struct ParseError {}

pub enum HtmlQuery {
    Attribute {
        key: String,
        value: String,
    },
    Name(String),
    Data(String),
    And {
        x: Box<HtmlQuery>,
        y: Box<HtmlQuery>,
    },
    Or {
        x: Box<HtmlQuery>,
        y: Box<HtmlQuery>,
    },
    Parent(Box<HtmlQuery>),
    Child(Box<HtmlQuery>),
    Sibling(Box<HtmlQuery>),
}

impl HtmlQuery {
    pub fn and(self, q: HtmlQuery) -> HtmlQuery {
        unimplemented!()
    }
}

pub fn find_by_rule(node: &Handle, query: &HtmlQuery) -> Result<Vec<Handle>, ParseError> {
    let matches_query = match query {
        HtmlQuery::Attribute { key, value } => has_attribute(&node, &key, &value),
        _ => unimplemented!(),
    };
    let mut result = vec![];
    if matches_query {
        let res = node.clone();
        result.push(res);
    }
    for child in node.children.borrow().iter() {
        let mut res = try!(find_by_rule(child, &query));
        result.append(&mut res);
    }
    Ok(result)
}

fn has_attribute(node: &Handle, key: &String, value: &String) -> bool {
    match node.data {
        NodeData::Element { ref attrs, .. } => {
            for attr in attrs.borrow().iter() {
                // TODO: upgrade to regex
                if attr.name.local.to_string() == *key && attr.value.to_string() == *value {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
