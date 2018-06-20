from lollygag_ext import query_html

some_html = """
           <asd-node style='boi'>
          asdkjaksdjaskjdjs
          </asd-node>
            <li>
            </li>
           """
print(query_html(some_html, "attr: style => boi"))
