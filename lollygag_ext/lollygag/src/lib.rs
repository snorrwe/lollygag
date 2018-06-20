extern crate html5ever;

use html5ever::rcdom::{Handle, NodeData};

#[cfg(test)]
mod test_html;

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
    use super::*;
    use html5ever::parse_document;
    use html5ever::rcdom::{Handle, NodeData, RcDom};
    use html5ever::tendril::TendrilSink;

    use test_html::html::*;

    fn get_handle(html: &str) -> Handle {
        let mut html = html.as_bytes();
        let handle = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut html)
            .unwrap();
        handle.document
    }

    fn assert_element(html: &str, query: &HtmlQuery, expected_num: usize, expected_tag: &str) {
        let handle = get_handle(html);

        let result = find_children_by_query(&handle, query);
        match result {
            Ok(result) => {
                assert_eq!(result.len(), expected_num);
                for res in result {
                    match res.data {
                        NodeData::Element { ref name, .. } => {
                            assert_eq!(name.local.to_string(), expected_tag.to_string())
                        }
                        _ => panic!("Unexpected node type!"),
                    };
                }
            }
            Err(e) => panic!(e),
        };
    }

    #[test]
    fn can_handle_invalid_html() {
        let query = HtmlQuery::Name("foo".to_string());
        assert_element(INVALID_TEST_HTML, &query, 1, "foo");
    }

    #[test]
    fn can_find_nodes_by_name_simple() {
        let query = HtmlQuery::Name("foo".to_string());
        assert_element(BASIC_TEST_HTML, &query, 1, "foo");
    }

    #[test]
    fn finds_all_code_nodes() {
        let query = HtmlQuery::Name("code".to_string());
        assert_element(LARGE_TEST_HTML, &query, 74, "code");
    }

    #[test]
    fn finds_nodes_by_attribute() {
        let query = HtmlQuery::Attribute {
            key: "bar".to_string(),
            value: "zoinks".to_string(),
        };
        assert_element(BASIC_TEST_HTML, &query, 1, "foo");
    }

    #[test]
    fn can_find_nodes_by_attribute_in_invalid_html() {
        let query = HtmlQuery::Attribute {
            key: "bar".to_string(),
            value: "zoinks".to_string(),
        };
        assert_element(INVALID_TEST_HTML, &query, 1, "foo");
    }
}
