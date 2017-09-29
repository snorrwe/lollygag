from lollygag.utility.url import get_domain
from lollygag.core.crawlers.crawler import Crawler


class DomainCrawlerStatus(object):
    __slots__ = ["domain", "visited_urls", "urls_to_crawl", "urls_in_progress"]

    def __init__(self, *args):
        assert not args or len(args) == 4
        if args:
            self.reset(*args)

    def reset(self, domain, visited_urls, urls_to_crawl, urls_in_progress):
        self.domain = domain
        self.visited_urls = visited_urls
        self.urls_to_crawl = urls_to_crawl
        self.urls_in_progress = urls_in_progress


class DomainCrawler(Crawler):
    """
    A crawler that does not go outside the domain boundaries of the initial url
    """

    def reset(self, url):
        self.status = DomainCrawlerStatus(None, set(), [], [])
        if not url:
            return
        if isinstance(url, list):
            self.initialize_url_list(url)
        else:
            self.initialize_url_list([url])

    def initialize_url_list(self, urls):
        self.status.domain = get_domain(urls[0])
        super(DomainCrawler, self).initialize_url_list(urls)

    def process_link(self, link):
        result = super(DomainCrawler, self).process_link(link)
        if get_domain(result) != self.status.domain:
            return None
        return result
