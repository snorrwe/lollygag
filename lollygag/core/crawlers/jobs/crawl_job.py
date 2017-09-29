import requests


class CrawlJob(object):
    """
    A single crawl job, that parses sites until it's crawler runs out of urls
    """

    def __init__(self, crawler):
        self.crawler = crawler

    def __call__(self):
        self.run()

    def run(self):
        """
        Parses sites until there are no more urls_to_crawl in the Crawler
        """
        parser = self.crawler.site_parser_factory()
        while self.crawler.status.urls_to_crawl:
            try:
                url = self.crawler.status.urls_to_crawl.pop()
            except IndexError:
                return
            self.crawler.status.urls_in_progress.append(url)
            result = self.crawl_site(url, parser)
            self.crawler.status.visited_urls.add(url)
            if result:
                self.crawler.process_links(url, result.links)
            self.crawler.log_service.debug(self.crawler.get_status_message())

    def crawl_site(self, url, crawler=None):
        """
        Crawls the given url using a crawler made by site_parser_factory
        If a requests.exceptions.ConnectionError
        or requests.exceptions.SSLError is raised
        returns None
        """
        try:
            crawler = self.crawler.site_parser_factory() if crawler is None else crawler
            result = crawler.crawl(url)
            return result
        except (requests.exceptions.ConnectionError, requests.exceptions.SSLError) as error:
            self.crawler.log_service.error(
                "Error while crawling site=[%s]" % url, str(error))
            return None
        finally:
            self.crawler.status.urls_in_progress.remove(url)
