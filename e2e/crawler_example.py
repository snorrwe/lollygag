#!/usr/bin/python

from frenetiq_crawler import run
from frenetiq_crawler.services import SERVICES
from frenetiq_crawler.crawler.link_crawler import LinkCrawler

class MyCrawler(LinkCrawler):
    """
    This is a custom crawler to handle a single webpage
    If you wish to preserve existing functionality when overriding methods,
    do not forget to call the super method aswell
    """
    def feed(self, data):
        # Override the feed method to customize handling the whole page data
        self.log_service.info("Yeah boi, a page!")
        return LinkCrawler.feed(self, data)

    def handle_starttag(self, tag, attrs):
        """
        Override HTMLParser methods for custom behaviour
        Call the super method to return links on the page
        The default DomainCrawler class will continue crawling if crawl() returns a 
        CrawlResult with links
        """
        super(MyCrawler, self).handle_starttag(tag, attrs)
        if(tag == 'img'):
            self.log_service.info("Yeeeeaaaah boiiiiiii, I found an %s" % (tag))
        if(tag == 'a'):
            self.log_service.debug("Boi, I found a link!")

def main():
    # Override crawler_factory with my own implementation
    SERVICES['crawler_factory'] = MyCrawler 
    run()

if __name__ == '__main__':
    main()
