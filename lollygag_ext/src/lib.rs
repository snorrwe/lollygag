extern crate html5ever;
#[macro_use]
extern crate cpython;

use cpython::{PyDict, PyErr, PyObject, PyResult, PyString, Python};
use html5ever::parse_document;
use html5ever::rcdom::NodeData;
use html5ever::rcdom::RcDom;
use html5ever::tendril::TendrilSink;

mod parsing;

use parsing::{find_children_by_query, HtmlQuery};

const QUERY_NONE: u8 = 0;
const QUERY_ATTRIBUTE: u8 = 1;
const QUERY_NAME: u8 = 2;

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

impl PyQuery {
    pub fn to_query(&self, py: Python) -> PyResult<HtmlQuery> {
        match *self.kind(py) {
            QUERY_NAME => {
                let string = try!(self.value(py).cast_as::<PyString>(py));
                let string = try!(string.to_string(py));
                Ok(HtmlQuery::Name(string.into_owned()))
            }
            QUERY_ATTRIBUTE => {
                let dict = try!(self.value(py).cast_as::<PyDict>(py));
                let name = try!(get_strs_from_dict(&dict, py, "name"));
                let value = try!(get_strs_from_dict(&dict, py, "value"));
                Ok(HtmlQuery::Attribute {
                    key: name,
                    value: value,
                })
            }
            QUERY_NONE => Ok(HtmlQuery::None),
            _ => unimplemented!(),
        }
    }
}

pub fn query_html(py: Python, html: PyString, query: PyObject) -> PyResult<String> {
    let query = try!(query.cast_as::<PyQuery>(py));
    let query = try!(query.to_query(py));
    let html = try!(html.to_string(py));
    let mut html = html.as_bytes();
    let handle = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html)
        .unwrap();
    let result = match find_children_by_query(&handle.document, &query) {
        Ok(result) => result,
        Err(_) => vec![],
    };
    for r in result {
        match r.data {
            NodeData::Text { ref contents } => println!("text boi\n{}", &contents.borrow()),
            NodeData::Element { ref name, .. } => println!("element boi\n{}", name.local),
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
        module.add(py, "QUERY_NONE", QUERY_NONE)?;
        module.add(py, "QUERY_NAME", QUERY_NAME)?;
        module.add(py, "QUERY_ATTRIBUTE", QUERY_ATTRIBUTE)?;
        module.add_class::<PyQuery>(py)?;
        Ok(())
    }
);
