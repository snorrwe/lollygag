use cpython::PythonObject;
use cpython::{PyList, PyResult, PyString, PyTuple, Python};
use html5ever::rcdom::{Handle, NodeData};
use html5ever::tendril::TendrilSink;

pub mod node_type {
    pub const UNKNOWN: u8 = 0;
    pub const DOCUMENT: u8 = 1;
    pub const ELEMENT: u8 = 2;
    pub const TEXT: u8 = 3;
}

py_class!(pub class HtmlNode |py| {
    data nodetype: u8;
    data name: Option<String>;
    data attributes: Option<PyList>;
    data text: Option<String>;
    data children: PyList;

    def __str__(&self) -> PyResult<String> {
        self.write(py, 0)
    }

    def __repr__(&self) -> PyResult<String> {
        let name = match self.name(py) {
            Some(name) => name,
            None => "",
        };
        let text = match self.text(py) {
            Some(t) => t,
            None => "",
        };
        Ok(format!("<HtmlNode type=[{}] name=[{}] text=[{}]>",
                   self.nodetype(py),          
                   name,
                   text.len()))
    }

    def write(&self, indent: usize = 0) -> PyResult<String> {
        let text = self.text(py);
        match text {
            Some(ref text) => {
                if text.len() > 0 {
                    return Ok(format!("{}{}\n", " ".repeat(indent), text));
                }
            },
            None => {},
        }
        let name = match self.name(py) {
            Some(name) => name,
            None => {
                return Ok("".to_string());
            }
        };
        let mut result = " ".repeat(indent);
        result += &format!("<{}", name);
        match self.attributes(py) {
            Some(a) => {
                for tuple in a.iter(py) {
                    let tuple = tuple.cast_as::<PyTuple>(py)?;
                    let (key,val) = (tuple.get_item(py, 0), tuple.get_item(py,1));
                    result += &format!(" {}=\"{}\"", key, val);
                }
            },
            None => {}
        }
        result += ">\n";
        for child in self.children(py).iter(py) {
            let child = child.cast_as::<HtmlNode>(py)?;
            result += &try!(child.write(py, indent+4));
        }
        result += &format!("{}</{}>\n", " ".repeat(indent), name);
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

    def get_type(&self) -> PyResult<u8> {
        Ok(*self.nodetype(py))
    }

    def __traverse__(&self, visit) {
        match self.attributes(py) {
            Some(a) => {visit.call(a);},
            None => {},
        }
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
                    node_type::ELEMENT,
                    Some(format!("{}", name.local)),
                    Some(attributes),
                    None,
                    children,
                )
            }
            NodeData::Text { ref contents, .. } => HtmlNode::create_instance(
                py,
                node_type::TEXT,
                None,
                None,
                Some(contents.borrow().trim().to_string()),
                PyList::new(py, &[]),
            ),
            NodeData::Document => HtmlNode::create_instance(
                py,
                node_type::DOCUMENT,
                None,
                None,
                None,
                PyList::new(py, &[]),
            ),
            _ => HtmlNode::create_instance(
                py,
                node_type::UNKNOWN,
                None,
                None,
                None,
                PyList::new(py, &[]),
            ),
        }
    }
}
