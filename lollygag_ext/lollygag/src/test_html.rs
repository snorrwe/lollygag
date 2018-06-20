#[cfg(test)]
pub mod html {

    pub const BASIC_TEST_HTML: &str = "
<div>
    <foo bar='zoinks'></foo>
</div>
";

    pub const INVALID_TEST_HTML: &str = "
<div>
    <li>
        <foo bar='zoinks'></foasdkjasko>
</div>
";

    pub const LARGE_TEST_HTML: &str = "
<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"utf-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <meta name=\"generator\" content=\"rustdoc\">
    <meta name=\"description\" content=\"API documentation for the Rust `PyErr` struct in crate `cpython`.\">
    <meta name=\"keywords\" content=\"rust, rustlang, rust-lang, PyErr\">

    <title>cpython::PyErr - Rust</title>

    <link rel=\"stylesheet\" type=\"text/css\" href=\"../normalize.css\">
    <link rel=\"stylesheet\" type=\"text/css\" href=\"../rustdoc.css\">
    <link rel=\"stylesheet\" type=\"text/css\" href=\"../main.css\">
    

    
    
</head>
<body class=\"rustdoc struct\">
    <!--[if lte IE 8]>
    <div class=\"warning\">
        This old browser is unsupported and will most likely display funky
        things.
    </div>
    <![endif]-->

    

    <nav class=\"sidebar\">
        <div class=\"sidebar-menu\">&#9776;</div>
        
        <p class='location'>Struct PyErr</p><div class=\"sidebar-elems\"><div class=\"block items\"><a class=\"sidebar-title\" href=\"#fields\">Fields</a><div class=\"sidebar-links\"><a href=\"#structfield.ptype\">ptype</a><a href=\"#structfield.pvalue\">pvalue</a><a href=\"#structfield.ptraceback\">ptraceback</a></div><a class=\"sidebar-title\" href=\"#methods\">Methods</a><div class=\"sidebar-links\"><a href=\"#method.new\">new</a><a href=\"#method.occurred\">occurred</a><a href=\"#method.new_type\">new_type</a><a href=\"#method.fetch\">fetch</a><a href=\"#method.from_instance\">from_instance</a><a href=\"#method.new_lazy_init\">new_lazy_init</a><a href=\"#method.print\">print</a><a href=\"#method.print_and_set_sys_last_vars\">print_and_set_sys_last_vars</a><a href=\"#method.matches\">matches</a><a href=\"#method.normalize\">normalize</a><a href=\"#method.get_type\">get_type</a><a href=\"#method.instance\">instance</a><a href=\"#method.restore\">restore</a><a href=\"#method.warn\">warn</a></div><a class=\"sidebar-title\" href=\"#implementations\">Trait Implementations</a><div class=\"sidebar-links\"><a href=\"#impl-Debug\">Debug</a><a href=\"#impl-PyDrop\">PyDrop</a><a href=\"#impl-PyClone\">PyClone</a><a href=\"#impl-From%3CPythonObjectDowncastError%3C%27p%3E%3E\">From&lt;PythonObjectDowncastError&lt;&#39;p&gt;&gt;</a></div></div><p class='location'><a href='index.html'>cpython</a></p><script>window.sidebarCurrent = {name: 'PyErr', ty: 'struct', relpath: ''};</script><script defer src=\"sidebar-items.js\"></script></div>
    </nav>

    <nav class=\"sub\">
        <form class=\"search-form js-only\">
            <div class=\"search-container\">
                <input class=\"search-input\" name=\"search\"
                       autocomplete=\"off\"
                       placeholder=\"Click or press ‘S’ to search, ‘?’ for more options…\"
                       type=\"search\">
            </div>
        </form>
    </nav>

    <section id='main' class=\"content\">
<h1 class='fqn'><span class='in-band'>Struct <a href='index.html'>cpython</a>::<wbr><a class=\"struct\" href=''>PyErr</a></span><span class='out-of-band'><span id='render-detail'>
                   <a id=\"toggle-all-docs\" href=\"javascript:void(0)\" title=\"collapse all docs\">
                       [<span class='inner'>&#x2212;</span>]
                   </a>
               </span><a class='srclink' href='../src/cpython/err.rs.html#125-137' title='goto source code'>[src]</a></span></h1>
<pre class='rust struct'>pub struct PyErr {
    pub ptype: <a class=\"struct\" href=\"../cpython/struct.PyObject.html\" title=\"struct cpython::PyObject\">PyObject</a>,
    pub pvalue: <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"../cpython/struct.PyObject.html\" title=\"struct cpython::PyObject\">PyObject</a>&gt;,
    pub ptraceback: <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"../cpython/struct.PyObject.html\" title=\"struct cpython::PyObject\">PyObject</a>&gt;,
}</pre><div class='docblock'><p>Represents a Python exception that was raised.</p>
</div><h2 id='fields' class='fields small-section-header'>
                       Fields<a href='#fields' class='anchor'></a></h2><span id=\"structfield.ptype\" class=\"structfield small-section-header\">
                           <a href=\"#structfield.ptype\" class=\"anchor field\"></a>
                           <span id=\"ptype.v\" class='invisible'>
                           <code>ptype: <a class=\"struct\" href=\"../cpython/struct.PyObject.html\" title=\"struct cpython::PyObject\">PyObject</a></code>
                           </span></span><div class='docblock'><p>The type of the exception. This should be either a <code>PyClass</code> or a <code>PyType</code>.</p>
</div><span id=\"structfield.pvalue\" class=\"structfield small-section-header\">
                           <a href=\"#structfield.pvalue\" class=\"anchor field\"></a>
                           <span id=\"pvalue.v\" class='invisible'>
                           <code>pvalue: <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"../cpython/struct.PyObject.html\" title=\"struct cpython::PyObject\">PyObject</a>&gt;</code>
                           </span></span><div class='docblock'><p>The value of the exception.</p>

<p>This can be either an instance of <code>ptype</code>,
a tuple of arguments to be passed to <code>ptype</code>&#39;s constructor,
or a single argument to be passed to <code>ptype</code>&#39;s constructor.
Call <code>PyErr::instance()</code> to get the exception instance in all cases.</p>
</div><span id=\"structfield.ptraceback\" class=\"structfield small-section-header\">
                           <a href=\"#structfield.ptraceback\" class=\"anchor field\"></a>
                           <span id=\"ptraceback.v\" class='invisible'>
                           <code>ptraceback: <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"../cpython/struct.PyObject.html\" title=\"struct cpython::PyObject\">PyObject</a>&gt;</code>
                           </span></span><div class='docblock'><p>The <code>PyTraceBack</code> object associated with the error.</p>
</div>
                    <h2 id='methods' class='small-section-header'>
                      Methods<a href='#methods' class='anchor'></a>
                    </h2>
                <h3 id='impl' class='impl'><span class='in-band'><code>impl <a class=\"struct\" href=\"../cpython/struct.PyErr.html\" title=\"struct cpython::PyErr\">PyErr</a></code><a href='#impl' class='anchor'></a></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#143-370' title='goto source code'>[src]</a></span></h3>
<div class='impl-items'><h4 id='method.new' class=\"method\"><span id='new.v' class='invisible'><code>pub fn <a href='#method.new' class='fnname'>new</a>&lt;T, V&gt;(py: <a class=\"struct\" href=\"../cpython/struct.Python.html\" title=\"struct cpython::Python\">Python</a>, value: V) -&gt; <a class=\"struct\" href=\"../cpython/struct.PyErr.html\" title=\"struct cpython::PyErr\">PyErr</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"../cpython/trait.PythonObjectWithTypeObject.html\" title=\"trait cpython::PythonObjectWithTypeObject\">PythonObjectWithTypeObject</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: <a class=\"trait\" href=\"../cpython/trait.ToPyObject.html\" title=\"trait cpython::ToPyObject\">ToPyObject</a>,&nbsp;</span></code></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#155-159' title='goto source code'>[src]</a></span></h4>
<div class='docblock'><p>Creates a new PyErr of type <code>T</code>.</p>

<p><code>value</code> can be:
* <code>NoArgs</code>: the exception instance will be created using python <code>T()</code>
* a tuple: the exception instance will be created using python <code>T(*tuple)</code>
* any other value: the exception instance will be created using python <code>T(value)</code></p>

<p>Panics if <code>T</code> is not a python class derived from <code>BaseException</code>.</p>

<p>Example:
 <code>return Err(PyErr::new::&lt;exc::TypeError, _&gt;(py, &quot;Error message&quot;));</code></p>
</div><h4 id='method.occurred' class=\"method\"><span id='occurred.v' class='invisible'><code>pub fn <a href='#method.occurred' class='fnname'>occurred</a>(_: <a class=\"struct\" href=\"../cpython/struct.Python.html\" title=\"struct cpython::Python\">Python</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></code></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#163-165' title='goto source code'>[src]</a></span></h4>
<div class='docblock'><p>Gets whether an error is present in the Python interpreter&#39;s global state.</p>
</div><h4 id='method.new_type' class=\"method\"><span id='new_type.v' class='invisible'><code>pub fn <a href='#method.new_type' class='fnname'>new_type</a>(<br>&nbsp;&nbsp;&nbsp;&nbsp;py: <a class=\"struct\" href=\"../cpython/struct.Python.html\" title=\"struct cpython::Python\">Python</a>, <br>&nbsp;&nbsp;&nbsp;&nbsp;name: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>, <br>&nbsp;&nbsp;&nbsp;&nbsp;base: <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"../cpython/struct.PyObject.html\" title=\"struct cpython::PyObject\">PyObject</a>&gt;, <br>&nbsp;&nbsp;&nbsp;&nbsp;dict: <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"../cpython/struct.PyObject.html\" title=\"struct cpython::PyObject\">PyObject</a>&gt;<br>) -&gt; <a class=\"struct\" href=\"../cpython/struct.PyType.html\" title=\"struct cpython::PyType\">PyType</a></code></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#172-190' title='goto source code'>[src]</a></span></h4>
<div class='docblock'><p>Creates a new exception type with the given name, which must be of the form
<code>&lt;module&gt;.&lt;ExceptionName&gt;</code>, as required by <code>PyErr_NewException</code>.</p>

<p><code>base</code> can be an existing exception type to subclass, or a tuple of classes
<code>dict</code> specifies an optional dictionary of class variables and methods</p>
</div><h4 id='method.fetch' class=\"method\"><span id='fetch.v' class='invisible'><code>pub fn <a href='#method.fetch' class='fnname'>fetch</a>(py: <a class=\"struct\" href=\"../cpython/struct.Python.html\" title=\"struct cpython::Python\">Python</a>) -&gt; <a class=\"struct\" href=\"../cpython/struct.PyErr.html\" title=\"struct cpython::PyErr\">PyErr</a></code></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#195-203' title='goto source code'>[src]</a></span></h4>
<div class='docblock'><p>Retrieves the current error from the Python interpreter&#39;s global state.
The error is cleared from the Python interpreter.
If no error is set, returns a <code>SystemError</code>.</p>
</div><h4 id='method.from_instance' class=\"method\"><span id='from_instance.v' class='invisible'><code>pub fn <a href='#method.from_instance' class='fnname'>from_instance</a>&lt;O&gt;(py: <a class=\"struct\" href=\"../cpython/struct.Python.html\" title=\"struct cpython::Python\">Python</a>, obj: O) -&gt; <a class=\"struct\" href=\"../cpython/struct.PyErr.html\" title=\"struct cpython::PyErr\">PyErr</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: <a class=\"trait\" href=\"../cpython/trait.PythonObject.html\" title=\"trait cpython::PythonObject\">PythonObject</a>,&nbsp;</span></code></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#233-235' title='goto source code'>[src]</a></span></h4>
<div class='docblock'><p>Creates a new PyErr.</p>

<p><code>obj</code> must be an Python exception instance, the PyErr will use that instance.
If <code>obj</code> is a Python exception type object, the PyErr will (lazily) create a new instance of that type.
Otherwise, a <code>TypeError</code> is created instead.</p>
</div><h4 id='method.new_lazy_init' class=\"method\"><span id='new_lazy_init.v' class='invisible'><code>pub fn <a href='#method.new_lazy_init' class='fnname'>new_lazy_init</a>(exc: <a class=\"struct\" href=\"../cpython/struct.PyType.html\" title=\"struct cpython::PyType\">PyType</a>, value: <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"../cpython/struct.PyObject.html\" title=\"struct cpython::PyObject\">PyObject</a>&gt;) -&gt; <a class=\"struct\" href=\"../cpython/struct.PyErr.html\" title=\"struct cpython::PyErr\">PyErr</a></code></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#263-269' title='goto source code'>[src]</a></span></h4>
<div class='docblock'><p>Construct a new error, with the usual lazy initialization of Python exceptions.
<code>exc</code> is the exception type; usually one of the standard exceptions like <code>py.get_type::&lt;exc::RuntimeError&gt;()</code>.
<code>value</code> is the exception instance, or a tuple of arguments to pass to the exception constructor.</p>
</div><h4 id='method.print' class=\"method\"><span id='print.v' class='invisible'><code>pub fn <a href='#method.print' class='fnname'>print</a>(self, py: <a class=\"struct\" href=\"../cpython/struct.Python.html\" title=\"struct cpython::Python\">Python</a>)</code></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#272-275' title='goto source code'>[src]</a></span></h4>
<div class='docblock'><p>Print a standard traceback to sys.stderr.</p>
</div><h4 id='method.print_and_set_sys_last_vars' class=\"method\"><span id='print_and_set_sys_last_vars.v' class='invisible'><code>pub fn <a href='#method.print_and_set_sys_last_vars' class='fnname'>print_and_set_sys_last_vars</a>(self, py: <a class=\"struct\" href=\"../cpython/struct.Python.html\" title=\"struct cpython::Python\">Python</a>)</code></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#278-281' title='goto source code'>[src]</a></span></h4>
<div class='docblock'><p>Print a standard traceback to sys.stderr.</p>
</div><h4 id='method.matches' class=\"method\"><span id='matches.v' class='invisible'><code>pub fn <a href='#method.matches' class='fnname'>matches</a>&lt;T&gt;(&amp;self, py: <a class=\"struct\" href=\"../cpython/struct.Python.html\" title=\"struct cpython::Python\">Python</a>, exc: T) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"../cpython/trait.ToPyObject.html\" title=\"trait cpython::ToPyObject\">ToPyObject</a>,&nbsp;</span></code></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#286-292' title='goto source code'>[src]</a></span></h4>
<div class='docblock'><p>Return true if the current exception matches the exception in <code>exc</code>.
If <code>exc</code> is a class object, this also returns <code>true</code> when <code>self</code> is an instance of a subclass.
If <code>exc</code> is a tuple, all exceptions in the tuple (and recursively in subtuples) are searched for a match.</p>
</div><h4 id='method.normalize' class=\"method\"><span id='normalize.v' class='invisible'><code>pub fn <a href='#method.normalize' class='fnname'>normalize</a>(&amp;mut self, py: <a class=\"struct\" href=\"../cpython/struct.Python.html\" title=\"struct cpython::Python\">Python</a>)</code></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#295-302' title='goto source code'>[src]</a></span></h4>
<div class='docblock'><p>Normalizes the error. This ensures that the exception value is an instance of the exception type.</p>
</div><h4 id='method.get_type' class=\"method\"><span id='get_type.v' class='invisible'><code>pub fn <a href='#method.get_type' class='fnname'>get_type</a>(&amp;self, py: <a class=\"struct\" href=\"../cpython/struct.Python.html\" title=\"struct cpython::Python\">Python</a>) -&gt; <a class=\"struct\" href=\"../cpython/struct.PyType.html\" title=\"struct cpython::PyType\">PyType</a></code></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#321-330' title='goto source code'>[src]</a></span></h4>
<div class='docblock'><p>Retrieves the exception type.</p>

<p>If the exception type is an old-style class, returns <code>oldstyle::PyClass</code>.</p>
</div><h4 id='method.instance' class=\"method\"><span id='instance.v' class='invisible'><code>pub fn <a href='#method.instance' class='fnname'>instance</a>(&amp;mut self, py: <a class=\"struct\" href=\"../cpython/struct.Python.html\" title=\"struct cpython::Python\">Python</a>) -&gt; <a class=\"struct\" href=\"../cpython/struct.PyObject.html\" title=\"struct cpython::PyObject\">PyObject</a></code></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#344-350' title='goto source code'>[src]</a></span></h4>
<div class='docblock'><p>Retrieves the exception instance for this error.
This method takes <code>&amp;mut self</code> because the error might need
to be normalized in order to create the exception instance.</p>
</div><h4 id='method.restore' class=\"method\"><span id='restore.v' class='invisible'><code>pub fn <a href='#method.restore' class='fnname'>restore</a>(self, py: <a class=\"struct\" href=\"../cpython/struct.Python.html\" title=\"struct cpython::Python\">Python</a>)</code></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#355-360' title='goto source code'>[src]</a></span></h4>
<div class='docblock'><p>Writes the error back to the Python interpreter&#39;s global state.
This is the opposite of <code>PyErr::fetch()</code>.</p>
</div><h4 id='method.warn' class=\"method\"><span id='warn.v' class='invisible'><code>pub fn <a href='#method.warn' class='fnname'>warn</a>(<br>&nbsp;&nbsp;&nbsp;&nbsp;py: <a class=\"struct\" href=\"../cpython/struct.Python.html\" title=\"struct cpython::Python\">Python</a>, <br>&nbsp;&nbsp;&nbsp;&nbsp;category: &amp;<a class=\"struct\" href=\"../cpython/struct.PyObject.html\" title=\"struct cpython::PyObject\">PyObject</a>, <br>&nbsp;&nbsp;&nbsp;&nbsp;message: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>, <br>&nbsp;&nbsp;&nbsp;&nbsp;stacklevel: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a><br>) -&gt; <a class=\"type\" href=\"../cpython/type.PyResult.html\" title=\"type cpython::PyResult\">PyResult</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>&gt;</code></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#364-369' title='goto source code'>[src]</a></span></h4>
<div class='docblock'><p>Issue a warning message.
May return a PyErr if warnings-as-errors is enabled.</p>
</div></div>
            <h2 id='implementations' class='small-section-header'>
              Trait Implementations<a href='#implementations' class='anchor'></a>
            </h2>
        <h3 id='impl-Debug' class='impl'><span class='in-band'><code>impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"../cpython/struct.PyErr.html\" title=\"struct cpython::PyErr\">PyErr</a></code><a href='#impl-Debug' class='anchor'></a></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#124' title='goto source code'>[src]</a></span></h3>
<div class='impl-items'><h4 id='method.fmt' class=\"method\"><span id='fmt.v' class='invisible'><code>fn <a href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt' class='fnname'>fmt</a>(&amp;self, __arg_0: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></code></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#124' title='goto source code'>[src]</a></span></h4>
<div class='docblock'><p>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></p>
</div></div><h3 id='impl-PyDrop' class='impl'><span class='in-band'><code>impl <a class=\"trait\" href=\"../cpython/trait.PyDrop.html\" title=\"trait cpython::PyDrop\">PyDrop</a> for <a class=\"struct\" href=\"../cpython/struct.PyErr.html\" title=\"struct cpython::PyErr\">PyErr</a></code><a href='#impl-PyDrop' class='anchor'></a></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#372-378' title='goto source code'>[src]</a></span></h3>
<div class='impl-items'><h4 id='method.release_ref' class=\"method\"><span id='release_ref.v' class='invisible'><code>fn <a href='../cpython/trait.PyDrop.html#tymethod.release_ref' class='fnname'>release_ref</a>(self, py: <a class=\"struct\" href=\"../cpython/struct.Python.html\" title=\"struct cpython::Python\">Python</a>)</code></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#373-377' title='goto source code'>[src]</a></span></h4>
</div><h3 id='impl-PyClone' class='impl'><span class='in-band'><code>impl <a class=\"trait\" href=\"../cpython/trait.PyClone.html\" title=\"trait cpython::PyClone\">PyClone</a> for <a class=\"struct\" href=\"../cpython/struct.PyErr.html\" title=\"struct cpython::PyErr\">PyErr</a></code><a href='#impl-PyClone' class='anchor'></a></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#380-388' title='goto source code'>[src]</a></span></h3>
<div class='impl-items'><h4 id='method.clone_ref' class=\"method\"><span id='clone_ref.v' class='invisible'><code>fn <a href='../cpython/trait.PyClone.html#tymethod.clone_ref' class='fnname'>clone_ref</a>(&amp;self, py: <a class=\"struct\" href=\"../cpython/struct.Python.html\" title=\"struct cpython::Python\">Python</a>) -&gt; <a class=\"struct\" href=\"../cpython/struct.PyErr.html\" title=\"struct cpython::PyErr\">PyErr</a></code></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#381-387' title='goto source code'>[src]</a></span></h4>
</div><h3 id='impl-From%3CPythonObjectDowncastError%3C%27p%3E%3E' class='impl'><span class='in-band'><code>impl&lt;'p&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"../cpython/struct.PythonObjectDowncastError.html\" title=\"struct cpython::PythonObjectDowncastError\">PythonObjectDowncastError</a>&lt;'p&gt;&gt; for <a class=\"struct\" href=\"../cpython/struct.PyErr.html\" title=\"struct cpython::PyErr\">PyErr</a></code><a href='#impl-From%3CPythonObjectDowncastError%3C%27p%3E%3E' class='anchor'></a></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#391-395' title='goto source code'>[src]</a></span></h3>
<div class='docblock'><p>Converts <code>PythonObjectDowncastError</code> to Python <code>TypeError</code>.</p>
</div><div class='impl-items'><h4 id='method.from' class=\"method\"><span id='from.v' class='invisible'><code>fn <a href='https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from' class='fnname'>from</a>(err: <a class=\"struct\" href=\"../cpython/struct.PythonObjectDowncastError.html\" title=\"struct cpython::PythonObjectDowncastError\">PythonObjectDowncastError</a>&lt;'p&gt;) -&gt; <a class=\"struct\" href=\"../cpython/struct.PyErr.html\" title=\"struct cpython::PyErr\">PyErr</a></code></span><span class='out-of-band'><div class='ghost'></div><a class='srclink' href='../src/cpython/err.rs.html#392-394' title='goto source code'>[src]</a></span></h4>
<div class='docblock'><p>Performs the conversion.</p>
</div></div></section>
    <section id='search' class=\"content hidden\"></section>

    <section class=\"footer\"></section>

    <aside id=\"help\" class=\"hidden\">
        <div>
            <h1 class=\"hidden\">Help</h1>

            <div class=\"shortcuts\">
                <h2>Keyboard Shortcuts</h2>

                <dl>
                    <dt>?</dt>
                    <dd>Show this help dialog</dd>
                    <dt>S</dt>
                    <dd>Focus the search field</dd>
                    <dt>↑</dt>
                    <dd>Move up in search results</dd>
                    <dt>↓</dt>
                    <dd>Move down in search results</dd>
                    <dt>↹</dt>
                    <dd>Switch tab</dd>
                    <dt>&#9166;</dt>
                    <dd>Go to active search result</dd>
                    <dt style=\"width:31px;\">+ / -</dt>
                    <dd>Collapse/expand all sections</dd>
                </dl>
            </div>

            <div class=\"infos\">
                <h2>Search Tricks</h2>

                <p>
                    Prefix searches with a type followed by a colon (e.g.
                    <code>fn:</code>) to restrict the search to a given type.
                </p>

                <p>
                    Accepted types are: <code>fn</code>, <code>mod</code>,
                    <code>struct</code>, <code>enum</code>,
                    <code>trait</code>, <code>type</code>, <code>macro</code>,
                    and <code>const</code>.
                </p>

                <p>
                    Search functions by type signature (e.g.
                    <code>vec -> usize</code> or <code>* -> vec</code>)
                </p>
            </div>
        </div>
    </aside>

    

    <script>
        window.rootPath = \"../\";
        window.currentCrate = \"cpython\";
    </script>
    <script src=\"../main.js\"></script>
    <script defer src=\"../search-index.js\"></script>
</body>
</html>
";
}
