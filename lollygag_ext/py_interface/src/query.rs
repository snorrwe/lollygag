use cpython::{PyObject, PyResult, PyString, Python};
use html5ever::parse_document;
use html5ever::rcdom::NodeData;
use html5ever::rcdom::RcDom;
use html5ever::tendril::TendrilSink;

use super::{PyQuery, PyQueryResult};
use lollygag::query_tree;

pub mod query {
    pub const QUERY_NONE: u8 = 0;
    pub const QUERY_ATTRIBUTE: u8 = 1;
    pub const QUERY_NAME: u8 = 2;
    pub const QUERY_OR: u8 = 3;
    pub const QUERY_AND: u8 = 4;
    pub const QUERY_DATA: u8 = 5;
}

/// Run a query on a single html file
/// Params:
/// html: str
/// query: PyQuery object
pub fn query_html(py: Python, html: PyString, query: PyObject) -> PyResult<PyQueryResult> {
    let query = try!(query.cast_as::<PyQuery>(py));
    let query = try!(query.to_query(py));
    let html = try!(html.to_string(py));
    let mut html = html.as_bytes();
    let handle = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html)
        .unwrap();
    let result = match query_tree(&handle.document, &query) {
        Ok(result) => result,
        Err(_) => vec![],
    };
    let found = result.len() > 0;
    for r in result {
        match r.data {
            NodeData::Text { ref contents } => println!("text boi\n{}", &contents.borrow()),
            NodeData::Element { ref name, .. } => println!("element boi <{}>", name.local),
            _ => println!("idk lol"),
        }
    }
    PyQueryResult::create_instance(py, found)
}
