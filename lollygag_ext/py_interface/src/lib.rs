#[macro_use]
extern crate cpython;
extern crate html5ever;

use cpython::{PyDict, PyObject, PyResult, PyString, PyTuple, Python};

extern crate lollygag;
mod query;
mod utils;

use lollygag::HtmlQuery;
use query::{query as query_consts, query_html};
use utils::get_strs_from_dict;

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
            query_consts::QUERY_NAME => {
                let string = try!(self.value(py).cast_as::<PyString>(py));
                let string = try!(string.to_string(py));
                Ok(HtmlQuery::Name(string.into_owned()))
            }
            query_consts::QUERY_ATTRIBUTE => {
                let dict = try!(self.value(py).cast_as::<PyDict>(py));
                let name = try!(get_strs_from_dict(&dict, py, "name"));
                let value = try!(get_strs_from_dict(&dict, py, "value"));
                Ok(HtmlQuery::Attribute {
                    key: name,
                    value: value,
                })
            }
            query_consts::QUERY_AND => binary_query!(And),
            query_consts::QUERY_OR => binary_query!(Or),
            query_consts::QUERY_NONE => Ok(HtmlQuery::None),
            query_consts::QUERY_DATA => {
                let string = try!(self.value(py).cast_as::<PyString>(py));
                let string = try!(string.to_string(py));
                Ok(HtmlQuery::Data(string.into_owned()))
            }
            _ => unimplemented!(),
        }
    }
}

py_class!(pub class PyQueryResult |py| {
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
        module.add(py, "QUERY_NONE", query_consts::QUERY_NONE)?;
        module.add(py, "QUERY_NAME", query_consts::QUERY_NAME)?;
        module.add(py, "QUERY_ATTRIBUTE", query_consts::QUERY_ATTRIBUTE)?;
        module.add(py, "QUERY_AND", query_consts::QUERY_AND)?;
        module.add(py, "QUERY_OR", query_consts::QUERY_OR)?;
        module.add(py, "QUERY_DATA", query_consts::QUERY_DATA)?;
        module.add_class::<PyQuery>(py)?;
        module.add_class::<PyQueryResult>(py)?;
        Ok(())
    }
);
