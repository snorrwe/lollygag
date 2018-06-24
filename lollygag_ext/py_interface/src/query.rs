use cpython::PythonObject;
use cpython::{PyDict, PyList, PyObject, PyResult, PyString, PyTuple, Python};
use html5ever::parse_document;
use html5ever::rcdom::RcDom;
use html5ever::rcdom::{Handle, NodeData};
use html5ever::tendril::TendrilSink;

use html_node::HtmlNode;
use lollygag::{query_tree, HtmlQuery};
use utils::get_strs_from_dict;

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
/// returns: List of nodes matching the query
pub fn query_html(py: Python, html: PyString, query: PyObject) -> PyResult<PyList> {
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
    let mut result_nodes: Vec<HtmlNode> = vec![];
    for res in result {
        result_nodes.push(try!(HtmlNode::new(py, &res)));
    }
    make_result(py, result_nodes)
}

fn make_result(py: Python, result: Vec<HtmlNode>) -> PyResult<PyList> {
    let mut mapped: Vec<PyObject> = vec![];
    for node in result {
        mapped.push(node.into_object());
    }
    let result = PyList::new(py, mapped.as_slice());
    Ok(result)
}

py_class!(pub class PyQuery |py| {
    data kind: u8;
    data value: PyObject;

    def __new__(_cls, kind: u8, value: PyObject) -> PyResult<PyQuery> {
        PyQuery::create_instance(py, kind, value)
    }

    def __repr__(&self) -> PyResult<String> {
        Ok(format!("<LollygagQueryProxyObject, kind: {}, value: {}>", self.kind(py), self.value(py)))
    }
});

impl PyQuery {
    pub fn to_query(&self, py: Python) -> PyResult<HtmlQuery> {
        macro_rules! binary_query {
            ($query:ident) => {{
                let list = try!(self.value(py).cast_as::<PyTuple>(py));
                let (x, y) = (
                    try!(list.get_item(py, 0).cast_as::<PyQuery>(py)?.to_query(py)),
                    try!(list.get_item(py, 1).cast_as::<PyQuery>(py)?.to_query(py)),
                );
                Ok(HtmlQuery::$query {
                    x: Box::new(x),
                    y: Box::new(y),
                })
            }};
        }

        match *self.kind(py) {
            query::QUERY_NAME => {
                let string = try!(self.value(py).cast_as::<PyString>(py));
                let string = try!(string.to_string(py));
                Ok(HtmlQuery::Name(string.into_owned()))
            }
            query::QUERY_ATTRIBUTE => {
                let dict = try!(self.value(py).cast_as::<PyDict>(py));
                let name = try!(get_strs_from_dict(&dict, py, "name"));
                let value = try!(get_strs_from_dict(&dict, py, "value"));
                Ok(HtmlQuery::Attribute {
                    key: name,
                    value: value,
                })
            }
            query::QUERY_AND => binary_query!(And),
            query::QUERY_OR => binary_query!(Or),
            query::QUERY_NONE => Ok(HtmlQuery::None),
            query::QUERY_DATA => {
                let string = try!(self.value(py).cast_as::<PyString>(py));
                let string = try!(string.to_string(py));
                Ok(HtmlQuery::Data(string.into_owned()))
            }
            _ => unimplemented!(),
        }
    }
}
