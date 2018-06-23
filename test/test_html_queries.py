from lollygag.query import compile_query, query_html

some_html = """
<asd-node style='boi' foo='bar'>
    <some-node foo='bar'>
        asdkjaksdjaskjdjs
    </some-node>
</asd-node>
<asd-node>
</asd-node>
<li>
</li>
"""


def test_or_query():
    query = compile_query({
        "or": ({
            "attribute": {
                "name": "foo",
                "value": ".*"
            },
            "name": "asd-node",
        }, {
            "data": "asd"
        })
    })
    result = query_html(some_html, query)
    assert result
    assert len(result) == 2
    def a(r):
        print(str(r))
        for c in r.get_children():
            a(c)
    list(a(i) for i in result)
    assert 0, result
