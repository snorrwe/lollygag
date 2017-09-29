import unittest
from lollygag.core.crawlers.crawler import Crawler


class CrawlerTests(unittest.TestCase):
    def test_can_create(self):
        self.assertTrue(Crawler())
        self.assertTrue(Crawler("https://someuri"))

    def test_init(self):
        urls = "https://fubar.com"
        crawler = Crawler(urls)
        self.__assert_status(crawler.status, set(), [urls], [])

    def __assert_status(self, status, visited_urls, urls_to_crawl, urls_in_progress):
        self.assertEqual(status.visited_urls, visited_urls)
        self.assertEqual(status.urls_to_crawl, urls_to_crawl)
        self.assertEqual(status.urls_in_progress, urls_in_progress)


if __name__ == '__main__':
    unittest.main()
