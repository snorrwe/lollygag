from lollygag_ext import query_html, PyQuery, QUERY_NAME, QUERY_ATTRIBUTE

some_html = """
<asd-node style='boi' foo='bar'>
    <some-node foo='bar'>
        asdkjaksdjaskjdjs
    </some-node>
</asd-node>
<li>
</li>
"""


query_html(some_html, PyQuery(QUERY_NAME, "asd-node"))

query = PyQuery(QUERY_ATTRIBUTE, {"name": "foo", "value": "bar"})
query_html(some_html, query)
