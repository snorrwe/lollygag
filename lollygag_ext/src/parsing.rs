use html5ever::rcdom::{Handle, NodeData};

pub struct ParseError {}

pub enum HtmlQuery {
    None,
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
    pub fn and(self, query: HtmlQuery) -> HtmlQuery {
        match self {
            HtmlQuery::None => return query,
            _ => HtmlQuery::And {
                x: Box::new(self),
                y: Box::new(query),
            },
        }
    }
}

pub fn find_children_by_query(node: &Handle, query: &HtmlQuery) -> Result<Vec<Handle>, ParseError> {
    let mut result = vec![];
    if query_node(node, query) {
        result.push(node.clone());
    }
    for child in node.children.borrow().iter() {
        let mut res = try!(find_children_by_query(child, &query));
        result.append(&mut res);
    }
    Ok(result)
}

pub fn query_node(node: &Handle, query: &HtmlQuery) -> bool {
    match query {
        HtmlQuery::Attribute { key, value } => has_attribute(&node, &key, &value),
        HtmlQuery::Name(query_name) => match node.data {
            NodeData::Element { ref name, .. } => name.local.to_string() == *query_name,
            _ => false,
        },
        HtmlQuery::None => false,
        _ => unimplemented!(),
    }
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
