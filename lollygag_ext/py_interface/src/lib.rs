extern crate html5ever;
#[macro_use]
extern crate cpython;

use cpython::{PyDict, PyObject, PyResult, PyString, Python};
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
}

py_class!(class KeyError |py| {});

pub fn get_strs_from_dict(dict: &PyDict, py: Python, key: &str) -> PyResult<String> {
    let pykey = PyString::new(py, key);
    match dict.get_item(py, pykey) {
        Some(string) => {
            let string = try!(string.cast_as::<PyString>(py));
            let string = try!(string.to_string(py));
            Ok(string.into_owned())
        }
        _ => panic!(format!("KeyError \"{}\"", key)),
    }
}

py_class!(class PyQuery |py| {

    data kind: u8;
    data value: PyObject;


    def __new__(_cls, kind: u8, value: PyObject) -> PyResult<PyQuery> {

        PyQuery::create_instance(py, kind, value)

    }

});

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
            _ => panic!(format!("KeyError")),
        };
        match result.cast_into::<TReturn>(py) {
            Ok(r) => Ok(r),
            Err(_) => panic!("Unexpected fatal error"),
        }
    }
}

impl PyQuery {
    pub fn to_query(&self, py: Python) -> PyResult<HtmlQuery> {
        macro_rules! binary_query {
            ($query:ident) => {{
                let dict = try!(self.value(py).cast_as::<PyDict>(py));
                let x = dict.get::<PyString, PyQuery>(py, &PyString::new(py, "x"))?
                    .to_query(py)?;
                let y = dict.get::<PyString, PyQuery>(py, &PyString::new(py, "y"))?
                    .to_query(py)?;
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
            _ => unimplemented!(),
        }
    }
}

/// Run a query on a single html file
/// Params:
/// html: str
/// query: PyQuery object
pub fn query_html(py: Python, html: PyString, query: PyObject) -> PyResult<String> {
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
    for r in result {
        match r.data {
            NodeData::Text { ref contents } => println!("text boi\n{}", &contents.borrow()),
            NodeData::Element { ref name, .. } => println!("element boi <{}>", name.local),
            _ => println!("idk lol"),
        }
    }
    Ok("done".to_string())
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
        module.add_class::<PyQuery>(py)?;
        Ok(())
    }
);
