from lollygag.utility.url import get_domain
from lollygag.core.crawlers.crawler import Crawler


class DomainCrawler(Crawler):
    """
    A crawler that does not go outside the domain boundaries of the initial url
    """

    def __init__(self, url=None):
        self.domain = None
        super(DomainCrawler, self).__init__(url)

    def initialize_url_list(self, urls):
        self.domain = get_domain(urls[0])
        super(DomainCrawler, self).initialize_url_list(urls)

    def process_link(self, origin, link):
        result = super(DomainCrawler, self).process_link(origin, link)
        if get_domain(result) != self.domain:
            return None
        return result

    def crawl(self, url=None):
        if url:
            self.reset(url)
        assert self.domain, "Cannot start crawling without a domain!"
        super(DomainCrawler, self).crawl(url)
