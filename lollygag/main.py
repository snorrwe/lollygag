import time
from lollygag.utility.url import get_domain
from lollygag.services import register_services
from lollygag.dependency_injection.inject import Inject


def run(**kwargs):
    register_services()
    config = Inject("config_service").request()
    config.setup()
    url = config.urls if "url" not in kwargs else kwargs["url"]
    if len(url) == 1:
        url = url[0]
    event_register = None
    if 'subscribe' in kwargs:
        def event_register(crawler):
            return register_events(crawler, **kwargs['subscribe'])
    if not url or not isinstance(url, list):
        crawler = get_crawler(event_register, **kwargs)
        crawler.crawl(url)
    else:
        crawl_url_list(url, event_register, **kwargs)


def crawl_url_list(url, event_register, **kwargs):
    work_service = Inject('work_service', cache=False).request()
    domains = separate_urls_by_domain(url)
    jobs = Inject("queue").request()
    for domain in domains:
        crawler = get_crawler(event_register, **kwargs)
        crawler.on_finish(lambda *a, **kw: jobs.get())
        job = get_crawl_job(crawler, domains[domain])
        work_service.request_work(job)
        jobs.put(1)
    while not jobs.empty():
        time.sleep(1)


def get_crawl_job(crawler, domain):
    return lambda: crawler.crawl_domain(domain)


def separate_urls_by_domain(urls):
    assert isinstance(urls, list)
    result = {}
    for url in urls:
        domain = get_domain(url)
        if domain not in result:
            result[domain] = []
        result[domain].append(url)
    return result


def get_crawler(event_register=None, **kwargs):
    result = None
    if "crawler" not in kwargs:
        result = Inject("domain_crawler_factory").request()
    else:
        result = kwargs["crawler"]()
    if event_register:
        event_register(result)
    return result


def register_events(crawler, **subscriptions):
    for key in subscriptions:
        assert key in ['on_start', 'on_interrupt', 'on_finish'],\
            "Unrecognised event type=[%s]" % key
        if isinstance(subscriptions[key], list):
            for sub in subscriptions[key]:
                register(crawler, key, sub)
        else:
            register(crawler, key, subscriptions[key])


def register(crawler, event_type, callback):
    assert crawler
    assert callable(callback)
    {
        'on_start': crawler.on_start,
        'on_finish': crawler.on_finish,
        'on_interrupt': crawler.on_interrupt
    }[event_type](callback)
