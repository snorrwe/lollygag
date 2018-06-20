from lollygag_ext import query_html, PyQuery, QUERY_NAME, QUERY_ATTRIBUTE

some_html = """
<asd-node style='boi' foo='bar'>
asdkjaksdjaskjdjs
</asd-node>
<li>
</li>
"""

print(query_html(some_html, PyQuery(QUERY_NAME, "asd-node")))
print(
    query_html(some_html,
               PyQuery(QUERY_ATTRIBUTE, {
                   "name": "foo",
                   "value": "bar"
               })))
