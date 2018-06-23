from time import time
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
start = time()
result = query_html(some_html, query)
delta = time() - start
print("boi", result, delta)
print(result.get_result())
