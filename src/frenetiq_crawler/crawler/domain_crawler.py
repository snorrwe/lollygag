import time
import requests
from frenetiq_crawler.crawler.url import get_protocol
from frenetiq_crawler.crawler.url import strip_beginning_slashes
from frenetiq_crawler.crawler.url import get_domain
from frenetiq_crawler.crawler.url import is_relative_link
from frenetiq_crawler.dependency_injection.inject import Inject
from frenetiq_crawler.dependency_injection.requirements import HasMethods, HasAttributes

class DomainCrawler(object):
    """
    Crawls a resource starting from url
    Uses Crawlers created by crawler_factory to crawl each resource,
    then decides the next pages to crawl based on the result
    Does not go outside the domain boundaries of the initial url

    Uses multiple working threads, based on the config_service.threads attribute
    Uses log_service to display results and errors
    """
    crawler_factory = Inject("crawler_factory", return_factory=True)
    log_service = Inject("log_service", HasMethods("info", "error", "debug"))
    work_service = Inject("work_service", \
                        HasMethods("request_work", "terminate_all", "active_count"))
    config_service = Inject("config_service", HasAttributes("url"))

    def __init__(self, url=None):
        if not url:
            url = self.config_service.url
        self.reset(url)

    def reset(self, url):
        self.domain = None
        self.visited_urls = set()
        self.urls_to_crawl = []
        self.urls_in_progress = []
        if not url:
            return
        self.protocol = get_protocol(url)
        if not self.protocol:
            self.protocol = "http://"
            url = "%s%s" % (self.protocol, url)
        self.domain = get_domain(url)
        self.urls_to_crawl.append(url)

    def crawl(self, url=None):
        """
        Starts the crawl procedure
        Returns when the crawling is complete or a KeyboardInterrupt or SystemExit is raised
        Joins working threads before returning
        """
        if url:
            self.reset(url)
        elif not self.domain:
            raise AssertionError("Cannot start crawling without a URL!")
        self.log_service.info("----------Crawl starting----------")
        self._init_crawl_thread()
        try:
            while self.is_task_left():
                self._run()
        except (KeyboardInterrupt, SystemExit) as error:
            self.log_service.info("Crawling was interrupted", error)
            self.work_service.terminate_all()
        finally:
            self.log_service.info(self.get_status_message())
            self.log_service.info("----------Crawl finished----------\n")

    def get_status_message(self):
        visited = len(self.visited_urls)
        in_progess = len(self.urls_in_progress)
        todo = len(self.urls_to_crawl)
        message = """
    Urls visited=[%s]
    Urls in progess=[%s]
    Urls left=[%s]""" % (visited, in_progess, todo)
        return message

    def _run(self):
        self.sleep_until_task_is_available()
        self._init_crawl_thread()

    def sleep_until_task_is_available(self):
        if not self.is_waiting_for_url():
            return
        self.log_service.debug("No urls to crawl, going to sleep."\
                             , "Work in progress=[%s]" % self.work_service.active_count())
        while self.is_waiting_for_url():
            time.sleep(1)

    def is_waiting_for_url(self):
        return not any(self.urls_to_crawl) \
              and self.work_service.active_count()

    def is_task_left(self):
        return any(self.urls_in_progress) or any(self.urls_to_crawl)

    def _init_crawl_thread(self):
        return self.work_service.request_work(self._run_crawl)

    def _run_crawl(self):
        try:
            url = self.urls_to_crawl.pop()
        except IndexError:
            return
        self.urls_in_progress.append(url)
        result = self.crawl_site(url)
        self.visited_urls.add(url)
        if result:
            self.process_links(result.links)
            self.log_service.debug(self.get_status_message())

    def crawl_site(self, url):
        """
        Crawls the given url using a crawler made by crawler_factory
        If a requests.exceptions.ConnectionError or requests.exceptions.SSLError is raised
        returns None
        """
        try:
            crawler = self.crawler_factory()
            result = crawler.crawl(url)
            return result
        except (requests.exceptions.ConnectionError, requests.exceptions.SSLError) as error:
            self.log_service.error("Error while crawling site", url, str(error))
            return None
        finally:
            self.urls_in_progress.remove(url)

    def process_links(self, links):
        for link in links:
            processed_link = self.process_link(link)
            if self.is_new_link(processed_link):
                self.urls_to_crawl.append(processed_link)

    def is_new_link(self, link):
        return link \
               and link not in self.visited_urls \
               and link not in self.urls_to_crawl

    def process_link(self, link):
        if not link or link[0] == "#":
            return None
        if is_relative_link(link):
            if link[0] == ".":
                link = link[1::]
            link = "%s%s" % (self.domain, link)
        link = strip_beginning_slashes(link)
        if get_domain(link) != self.domain:
            return None
        if not get_protocol(link):
            link = "%s%s" % (self.protocol, link)
        return link
