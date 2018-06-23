#[macro_use]
extern crate cpython;
extern crate html5ever;

use cpython::{PyDict, PyObject, PyResult, PyString, Python};

extern crate lollygag;
mod query;
mod utils;

use query::{query as query_consts, query_html, HtmlNode, PyQuery};

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
        module.add_class::<HtmlNode>(py)?;
        Ok(())
    }
);
