extern crate html5ever;
#[macro_use]
extern crate cpython;

use cpython::{PyObject, PyResult, PyString, Python};
use html5ever::parse_document;
use html5ever::rcdom::NodeData;
use html5ever::rcdom::{Handle, RcDom};
use html5ever::tendril::TendrilSink;

mod parsing;

use parsing::{find_by_rule, HtmlQuery};

pub fn query_html(py: Python, html: PyString, query: PyString) -> PyResult<String> {
    let query = HtmlQuery::Attribute {
        key: "style".to_string(),
        value: "boi".to_string(),
    };
    let html = try!(html.to_string(py));
    let mut html = html.as_bytes();
    let handle = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html)
        .unwrap();
    let result = match find_by_rule(&handle.document, &query) {
        Ok(result) => result,
        Err(_) => vec![],
    };
    for r in result {
        match r.data {
            NodeData::Text { ref contents } => println!("text boi\n{}\n", &contents.borrow()),
            NodeData::Element { ref name, .. } => println!("element boi\n{}\n", name.local),
            _ => println!("idk lol"),
        }
    }

    Ok("".to_string())
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
            py_fn!(py, query_html(html: PyString, query: PyString)),
        )?;
        Ok(())
    }
);
