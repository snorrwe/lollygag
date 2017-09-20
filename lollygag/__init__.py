from lollygag.services import register_services, Services

def run(**kwargs):
    register_services()
    crawler = Services.domain_crawler_factory() if "crawler" not in kwargs else kwargs["crawler"]
    url = None if "url" not in kwargs else kwargs["url"]
    if 'subscribe' in kwargs:
        register_events(crawler, **kwargs['subscribe'])
    crawler.crawl_domain(url)

def register(crawler, type, callback):
    {
        'on_start': lambda x: crawler.on_start(x)
        , 'on_finish': lambda x: crawler.on_finish(x)
        , 'on_interrupt': lambda x: crawler.on_interrupt(x)
    }[type](callback)

def register_events(crawler, **subscriptions):
    for key in subscriptions:
        assert key in ['on_start', 'on_interrupt', 'on_finish'],\
                         "Unrecognised event type=[%s]" % key
        if isinstance(subscriptions[key], list):
            for sub in subscriptions[key]:
                register(crawler, key, sub)
        else:
            register(crawler, key, subscriptions[key])
