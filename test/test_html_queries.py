from lollygag.query import compile_query, query_html

some_html = """
<asd-node style="boi" foo="bar">   <some-node foo="baz">
                          asdkjaksdjaskjdjs
</asd-node>
<asd-node>
                            </asd-node>
<li>
    <i>
        some asd data   </i>
</li>
"""


def test_data_query_regex():
    html = """
<ul>
    <li>
        something interesting
    </li>
    <li>
        some item
    </li>
    <li>
        something interesting
    </li>
    <li>
        some item
    </li>
    <li>
        something interesting
    </li>
</ul>
    """
    query = compile_query({"data": "inter.*"})
    result = query_html(html, query)
    assert result
    assert len(result) == 3
    for r in result:
        assert str(r) == """<li>
    something interesting
</li>
"""


def test_name_query_regex():
    query = compile_query({"name": "some.*"})
    result = query_html(some_html, query)
    assert result
    assert len(result) == 1

    res = str(result[0])
    print(res)
    assert res == """<some-node foo="baz">
    asdkjaksdjaskjdjs
</some-node>
"""


def test_attribute_query_regex():
    query = compile_query({"attribute": {"name": "foo", "value": "ba."}})
    result = query_html(some_html, query)
    assert result
    assert len(result) == 2

    res = str(result[0])
    print(res)
    assert res == """<asd-node style="boi" foo="bar">
    <some-node foo="baz">
        asdkjaksdjaskjdjs
    </some-node>
</asd-node>
"""

    res = str(result[1])
    print(res)
    assert res == """<some-node foo="baz">
    asdkjaksdjaskjdjs
</some-node>
"""


def test_attribute_query():
    query = compile_query({"attribute": {"name": "foo", "value": "bar"}})
    result = query_html(some_html, query)
    assert result
    assert len(result) == 1

    res = str(result[0])
    print(res)
    assert res == """<asd-node style="boi" foo="bar">
    <some-node foo="baz">
        asdkjaksdjaskjdjs
    </some-node>
</asd-node>
"""


def test_or_query():
    query = compile_query({
        "or": ({
            "attribute": {
                "name": "foo",
                "value": "bar"
            },
            "name": "asd-node",
        }, {
            "data": "asd"
        })
    })
    result = query_html(some_html, query)
    assert result
    assert len(result) == 3

    res = str(result[0])
    print(res)
    assert res == """<asd-node style="boi" foo="bar">
    <some-node foo="baz">
        asdkjaksdjaskjdjs
    </some-node>
</asd-node>
"""
    res = str(result[1])
    print(res)
    assert res == """<some-node foo="baz">
    asdkjaksdjaskjdjs
</some-node>
"""
    res = str(result[2])
    print(res)
    assert res == """<i>
    some asd data
</i>
"""
