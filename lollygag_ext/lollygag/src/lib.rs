extern crate html5ever;
extern crate regex;

use html5ever::rcdom::{Handle, NodeData};
use regex::Regex;

mod query;

pub use query::HtmlQuery;

pub struct ParseError {}

/// Get all nodes in a html tree matching the query
/// Returns a vector of matching nodes
pub fn query_tree(node: &Handle, query: &HtmlQuery) -> Result<Vec<Handle>, ParseError> {
    let mut result = vec![];
    if matches_node(node, query) {
        result.push(node.clone());
    }
    for child in node.children.borrow().iter() {
        let mut res = try!(query_tree(child, &query));
        result.append(&mut res);
    }
    Ok(result)
}

/// Check if the html node matches the query
/// `HtmlQuery::None` does not match any nodes
pub fn matches_node(node: &Handle, query: &HtmlQuery) -> bool {
    match query {
        HtmlQuery::Attribute { key, value } => has_attribute(&node, &key, &value),
        HtmlQuery::Data(pattern) => matches_data(node, &pattern),
        HtmlQuery::Name(query_name) => match node.data {
            NodeData::Element { ref name, .. } => Regex::new(query_name)
                .unwrap()
                .is_match(&name.local.to_string()),
            _ => false,
        },
        HtmlQuery::And { x, y } => matches_node(&node, &x) && matches_node(&node, &y),
        HtmlQuery::Or { x, y } => matches_node(&node, &x) || matches_node(&node, &y),
        HtmlQuery::None => false,
        _ => unimplemented!(),
    }
}

/// Check if the html node has an attribute named `key` with value of `value`
pub fn has_attribute(node: &Handle, key: &String, value: &String) -> bool {
    match node.data {
        NodeData::Element { ref attrs, .. } => {
            let key = Regex::new(key).unwrap();
            let value = Regex::new(value).unwrap();
            for attr in attrs.borrow().iter() {
                if key.is_match(&attr.name.local.to_string())
                    && value.is_match(&attr.value.to_string())
                {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}

/// Check if the pattern is found in the `Text` node
/// Other types of nodes are ignored
pub fn matches_data(node: &Handle, pattern: &String) -> bool {
    match node.data {
        NodeData::Element { .. } => {
            let re = Regex::new(pattern).unwrap();
            for child in node.children.borrow().iter() {
                match child.data {
                    NodeData::Text { ref contents, .. } => {
                        if re.is_match(&contents.borrow()) {
                            return true;
                        }
                    }
                    _ => (),
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

    use std::fs::File;
    use std::io::Read;

    use html5ever::parse_document;
    use html5ever::rcdom::{Handle, NodeData, RcDom};
    use html5ever::tendril::TendrilSink;

    const BASIC_TEST_HTML: &str = "test/data/basic.html";
    const INVALID_TEST_HTML: &str = "test/data/invalid.html";
    const LARGE_TEST_HTML: &str = "test/data/pyerr.html";
    const FORGE_HTML: &str = "test/data/rust_forge.html";

    fn get_handle(html_path: &str) -> std::io::Result<Handle> {
        let mut html_file = File::open(html_path)?;
        let mut html = String::new();
        html_file.read_to_string(&mut html)?;
        let mut html = html.as_bytes();
        let handle = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut html)
            .unwrap();
        Ok(handle.document)
    }

    /// Asserts if the query finds nodes in the html
    /// Note: html file paths are relative to the lollygag dir make sure your cwd is correct!
    fn assert_element(html: &str, query: &HtmlQuery, expected_num: usize, expected_tag: &str) {
        let handle = match get_handle(html) {
            Ok(r) => r,
            Err(e) => panic!(e),
        };

        let result = query_tree(&handle, query);
        match result {
            Ok(result) => {
                assert_eq!(result.len(), expected_num);
                for res in result {
                    match res.data {
                        NodeData::Element { ref name, .. } => {
                            assert_eq!(name.local.to_string(), expected_tag.to_string())
                        }
                        NodeData::Text { .. } => {}
                        _ => unimplemented!(),
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
    fn can_find_nodes_by_name_regex() {
        let query = HtmlQuery::Name("^f.*".to_string());
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

    #[test]
    fn can_find_nodes_by_attribute_regex() {
        let query = HtmlQuery::Attribute {
            key: "bar".to_string(),
            value: "z.{4}s".to_string(),
        };
        assert_element(BASIC_TEST_HTML, &query, 1, "foo");
    }

    #[test]
    fn can_find_nodes_by_name_and_attribute() {
        let query = HtmlQuery::Attribute {
            key: "bar".to_string(),
            value: "z.{4}s".to_string(),
        };
        let query = query.and(HtmlQuery::Name("foo".to_string()));
        assert_element(BASIC_TEST_HTML, &query, 1, "foo");
    }

    #[test]
    fn can_find_nodes_by_name_and_attribute_no_match() {
        let query = HtmlQuery::Attribute {
            key: "bar".to_string(),
            value: "z.{4}s".to_string(),
        };
        let query = query.and(HtmlQuery::Name("fooq".to_string()));
        assert_element(BASIC_TEST_HTML, &query, 0, "");
    }

    #[test]
    fn can_find_nodes_by_name_or_attribute() {
        let query = HtmlQuery::Attribute {
            key: "bar".to_string(),
            value: "z.{4}s".to_string(),
        };
        let query = query.or(HtmlQuery::Name("foo".to_string()));
        assert_element(BASIC_TEST_HTML, &query, 1, "foo");
    }

    #[test]
    fn can_find_nodes_by_name_or_attribute_lhs_only() {
        let query = HtmlQuery::Attribute {
            key: "bar".to_string(),
            value: "z.{4}s".to_string(),
        };
        let query = query.or(HtmlQuery::Name("fooq".to_string()));
        assert_element(BASIC_TEST_HTML, &query, 1, "foo");
    }

    #[test]
    fn can_find_nodes_by_name_or_attribute_rhs_only() {
        let query = HtmlQuery::Attribute {
            key: "barasd".to_string(),
            value: ".*".to_string(),
        };
        let query = query.or(HtmlQuery::Name("foo".to_string()));
        assert_element(BASIC_TEST_HTML, &query, 1, "foo");
    }

    #[test]
    fn find_nodes_by_name_or_attribute_no_match() {
        let query = HtmlQuery::Attribute {
            key: "barasd".to_string(),
            value: ".*".to_string(),
        };
        let query = query.or(HtmlQuery::Name("fooq".to_string()));
        assert_element(BASIC_TEST_HTML, &query, 0, "");
    }

    #[test]
    fn finds_text_data_by_pattern() {
        let query = HtmlQuery::Data(r"Rust".to_string());
        assert_element(LARGE_TEST_HTML, &query, 1, "");
    }

    #[test]
    fn finds_text_data_by_pattern_2() {
        let query = HtmlQuery::Data(r"Rust".to_string());
        assert_element(FORGE_HTML, &query, 11, "");
    }
}
