from collections import namedtuple
import abc
from frenetiq_crawler.dependency_injection.inject import Inject
from frenetiq_crawler.dependency_injection.requirements import HasMethods

CrawlResult = namedtuple("CrawlResult", ["link", "status_code", "page_size"])

class Crawler(object):
    """
    Base class for crawlers
    crawl performs a GET rquest on the url and calls feed with the response.text
    Subclasses should implement the feed method
    """
    __metaclass__ = abc.ABCMeta
    _requests = Inject("requests", HasMethods("get"))
    log_service = Inject("log_service", HasMethods("debug", "info", "error", "warn"))

    def crawl(self, url):
        assert url is not None
        response = self._requests.get(url, verify=False)
        if response.status_code == 200:
            self.feed(response.text)
        return CrawlResult(link=url, status_code=response.status_code \
            , page_size=len(response.content))

    def feed(self, data):
        pass