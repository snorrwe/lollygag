from collections import namedtuple
from lollygag.core.single_site.crawler_base import Crawler
try:
    from HTMLParser import HTMLParser
except ImportError:
    from html.parser import HTMLParser

CrawlResult = namedtuple("CrawlResult", ["link", "status_code", "page_size", "links"])

class LinkCrawler(HTMLParser, Crawler):
    """
    A website crawler that collects links from href attributes on the site
    """
    def __init__(self, *args, **kwargs):
        super(LinkCrawler, self).__init__(*args, **kwargs)
        self._links = set()

    def crawl(self, url):
        """
        Performs a GET request on the resource and collects links in href attributes
        """
        self._links = set()
        result = Crawler.crawl(self, url)
        self.log_service.info("Link=[%s] StatusCode=[%s] Size=[%s]"\
            % (result.link, result.status_code, result.page_size))
        return CrawlResult(link=result.link, status_code=result.status_code \
            , page_size=result.page_size, links=self._links)

    def feed(self, data):
        return HTMLParser.feed(self, data)

    def handle_starttag(self, tag, attrs):
        if tag != 'a':
            return
        for attribute in attrs:
            if attribute[0] == "href":
                self._links.add(attribute[1])
