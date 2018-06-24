use cpython::PythonObject;
use cpython::{PyDict, PyList, PyObject, PyResult, PyString, PyTuple, Python};
use html5ever::parse_document;
use html5ever::rcdom::RcDom;
use html5ever::rcdom::{Handle, NodeData};
use html5ever::tendril::TendrilSink;

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

py_class!(pub class HtmlNode |py| {
    data name: String;
    data attributes: PyList;
    data text: String;
    data children: PyList;

    def __str__(&self) -> PyResult<String> {
        self.write(py, 0)
    }

    def __repr__(&self) -> PyResult<String> {
        Ok(format!("<HtmlNode name=[{}] text=[{}]>",
                   self.name(py),
                   self.text(py).len()))
    }

    def write(&self, indent: usize = 0) -> PyResult<String> {
        let text = self.text(py);
        if !text.is_empty()  {
            return Ok(format!("{}{}\n", " ".repeat(indent), text));
        }
        let name = self.name(py);
        if name.is_empty() {
            return Ok("".to_string());
        }
        let mut result = " ".repeat(indent);
        result += &format!("<{}", name);
        for tuple in self.attributes(py).iter(py) {
            let tuple = tuple.cast_as::<PyTuple>(py)?;
            let (key,val) = (tuple.get_item(py, 0), tuple.get_item(py,1));
            result += &format!(" {}=\"{}\"", key, val);
        }
        result += ">\n";
        for child in self.children(py).iter(py) {
            let child = child.cast_as::<HtmlNode>(py)?;
            result += &try!(child.write(py, indent+4));
        }
        result += &format!("{}</{}>\n", " ".repeat(indent), self.name(py));
        Ok(result)
    }

    def get_children(&self) -> PyResult<PyList> {
        let children = self.children(py);
        let mut result = vec![];
        for child in self.children(py).iter(py) {
            result.push(child);
        }
        Ok(PyList::new(py, result.as_slice()))
    }

    def __traverse__(&self, visit) {
        visit.call(self.attributes(py));
        visit.call(self.children(py));
        Ok(())
    }

    def __clear__(&self) {
    }
});

impl HtmlNode {
    pub fn new(py: Python, rc_handle: &Handle) -> PyResult<HtmlNode> {
        match rc_handle.data {
            NodeData::Element {
                ref name,
                ref attrs,
                ..
            } => {
                let mut children = vec![];
                for child in rc_handle.children.borrow().iter() {
                    children.push(HtmlNode::new(py, child)?.into_object());
                }
                let mut attributes = vec![];
                for attr in attrs.borrow().iter() {
                    let attr = vec![
                        PyString::new(py, &attr.name.local.to_string()).into_object(),
                        PyString::new(py, &attr.value.to_string()).into_object(),
                    ];
                    attributes.push(PyTuple::new(py, attr.as_slice()).into_object());
                }
                let attributes = PyList::new(py, attributes.as_slice());
                let children = PyList::new(py, children.as_slice());
                HtmlNode::create_instance(
                    py,
                    format!("{}", name.local),
                    attributes,
                    "".to_string(),
                    children,
                )
            }
            NodeData::Text { ref contents, .. } => HtmlNode::create_instance(
                py,
                "".to_string(),
                PyList::new(py, &[]),
                contents.borrow().trim().to_string(),
                PyList::new(py, &[]),
            ),
            _ => unimplemented!(),
        }
    }
}
