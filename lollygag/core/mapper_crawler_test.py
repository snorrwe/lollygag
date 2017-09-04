import unittest
from lollygag.core.domain_crawler_test import DomainCrawlerTests
from lollygag.core.mapper_crawler import MapperCrawler
from lollygag.dependency_injection.inject import Inject
from lollygag.utility.test_utils import Any, CallableMock

class MapperCrawlerTests(DomainCrawlerTests):
    def setUp(self):
        super(MapperCrawlerTests, self).setUp()

    def tearDown(self):
        super(MapperCrawlerTests, self).tearDown()

if __name__ == '__main__':
    unittest.main()
