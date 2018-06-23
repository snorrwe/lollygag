extern crate html5ever;
#[macro_use]
extern crate cpython;

use cpython::{PyDict, PyObject, PyResult, PyString, PyTuple, Python};
use html5ever::parse_document;
use html5ever::rcdom::NodeData;
use html5ever::rcdom::RcDom;
use html5ever::tendril::TendrilSink;

extern crate lollygag;

use lollygag::{query_tree, HtmlQuery};

pub mod query {
    pub const QUERY_NONE: u8 = 0;
    pub const QUERY_ATTRIBUTE: u8 = 1;
    pub const QUERY_NAME: u8 = 2;
    pub const QUERY_OR: u8 = 3;
    pub const QUERY_AND: u8 = 4;
    pub const QUERY_DATA: u8 = 5;
}

pub fn get_strs_from_dict(dict: &PyDict, py: Python, key: &str) -> PyResult<String> {
    let pykey = PyString::new(py, key);
    match dict.get_item(py, pykey) {
        Some(string) => {
            let string = try!(string.cast_as::<PyString>(py));
            let string = try!(string.to_string(py));
            Ok(string.into_owned())
        }
        _ => panic!(format!("KeyError \"{}\"", key)), // FIXME: return proper error
    }
}

trait Getter {
    fn get<TKey, TReturn>(&self, py: Python, key: &TKey) -> PyResult<TReturn>
    where
        TKey: cpython::ToPyObject,
        TReturn: cpython::PythonObjectWithCheckedDowncast;
}

impl Getter for PyDict {
    fn get<TKey, TReturn>(&self, py: Python, key: &TKey) -> PyResult<TReturn>
    where
        TKey: cpython::ToPyObject,
        TReturn: cpython::PythonObjectWithCheckedDowncast,
    {
        let result = match self.get_item(py, key) {
            Some(val) => val,
            _ => panic!(format!("KeyError")), // FIXME: return proper error
        };
        Ok(result.cast_into::<TReturn>(py).unwrap())
    }
}

py_class!(class PyQuery |py| {
    data kind: u8;
    data value: PyObject;

    def __new__(_cls, kind: u8, value: PyObject) -> PyResult<PyQuery> {
        PyQuery::create_instance(py, kind, value)
    }

    def __repr__(&self) -> PyResult<String> {
        Ok(format!("<LollygagQueryProxyObject>, kind: {}, value: {}", self.kind(py), self.value(py)))
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

py_class!(class PyQueryResult |py| {
    data result: bool;

    def __new__(_cls, result: bool) -> PyResult<PyQueryResult > {
        PyQueryResult::create_instance(py, result)
    }

    def __repr__(&self) -> PyResult<String> {
        Ok(format!("<LollygagQueryResultProxyObject>, result: {}", self.result(py)))
    }

    def get_result(&self) -> PyResult<bool> {
        Ok(*self.result(py))
    }
});

/// Run a query on a single html file
/// Params:
/// html: str
/// query: PyQuery object
fn query_html(py: Python, html: PyString, query: PyObject) -> PyResult<PyQueryResult> {
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

py_module_initializer!(
    lollygag_ext,
    init_lollygag_ext,
    PyInit_lollygag_ext,
    |py, module| {
        module.add(py, "__doc__", "Rust extension for lollygag")?;
        module.add(
            py,
            "query_html",
            py_fn!(py, query_html(html: PyString, query: PyObject)),
        )?;
        module.add(py, "QUERY_NONE", query::QUERY_NONE)?;
        module.add(py, "QUERY_NAME", query::QUERY_NAME)?;
        module.add(py, "QUERY_ATTRIBUTE", query::QUERY_ATTRIBUTE)?;
        module.add(py, "QUERY_AND", query::QUERY_AND)?;
        module.add(py, "QUERY_OR", query::QUERY_OR)?;
        module.add(py, "QUERY_DATA", query::QUERY_DATA)?;
        module.add_class::<PyQuery>(py)?;
        module.add_class::<PyQueryResult>(py)?;
        Ok(())
    }
);
