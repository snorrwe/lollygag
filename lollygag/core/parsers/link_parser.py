from collections import namedtuple
from lollygag.core.parsers.parser_base import Parser
try:
    from HTMLParser import HTMLParser
except ImportError:
    from html.parser import HTMLParser

ParseResult = namedtuple("ParseResult", ["link", "status_code", "page_size", "links"])


class LinkParser(HTMLParser, Parser):
    """
    A website parser that collects links from href attributes on the site
    """

    def __init__(self, *args, **kwargs):
        super(LinkParser, self).__init__(*args, **kwargs)
        self._links = set()

    def crawl(self, url):
        """
        Performs a GET request on the resource and collects links in href attributes
        """
        self._links = set()
        result = Parser.crawl(self, url)
        self.log_service.info("Link=[%s] StatusCode=[%s] Size=[%s]"
                              % (result.link, result.status_code, result.page_size))
        return ParseResult(link=result.link,
                           status_code=result.status_code,
                           page_size=result.page_size,
                           links=self._links)

    def feed(self, data):
        return HTMLParser.feed(self, data)

    def handle_starttag(self, tag, attrs):
        if tag != 'a':
            return
        for attribute in attrs:
            if attribute[0] == "href":
                self._links.add(attribute[1])
