import unittest
from lollygag.core.crawlers.crawler import Crawler


class CrawlerTests(unittest.TestCase):
    def test_can_create(self):
        self.assertTrue(Crawler())
        self.assertTrue(Crawler("https://someuri"))

    def test_init(self):
        urls = ["https://fubar.com"]
        crawler = Crawler(urls[0])
        self.__assert_status(crawler.status, set(), set(urls), [])

    def __assert_status(self, status, visited_urls, urls_to_crawl, urls_in_progress):
        self.assertEqual(status.visited_urls, visited_urls)
        self.assertEqual(status.urls_to_crawl, urls_to_crawl)
        self.assertEqual(status.urls_in_progress, urls_in_progress)

    def test_initialize_status_no_protocol(self):
        crawler = Crawler()
        urls = ["winnie"]
        crawler.initialize_status(urls)
        self.assertEqual(crawler.protocol, "http://")

    def test_initialize_status_http(self):
        crawler = Crawler()
        urls = ["http://winnie"]
        crawler.initialize_status(urls)
        self.assertEqual(crawler.protocol, "http://")

    def test_initialize_status_https(self):
        crawler = Crawler()
        urls = ["https://winnie"]
        crawler.initialize_status(urls)
        self.assertEqual(crawler.protocol, "https://")

    def test_initialize_status_inits_status_correctly(self):
        crawler = Crawler()
        urls = ["http://a", "http://b", "http://c"]
        crawler.initialize_status(urls)
        self.__assert_status(crawler.status, set(), set(urls), [])


if __name__ == '__main__':
    unittest.main()
