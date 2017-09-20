from lollygag.services import register_services, Services

def run(**kwargs):
    register_services()
    crawler = Services.domain_crawler_factory() if "crawler" not in kwargs else kwargs["crawler"]
    url = None if "url" not in kwargs else kwargs["url"]
    if 'subscribe' in kwargs:
        register_events(crawler, **kwargs['subscribe'])
    crawler.crawl_domain(url)

def register(crawler, event_type, callback):
    {
        'on_start': crawler.on_start,
        'on_finish': crawler.on_finish,
        'on_interrupt': crawler.on_interrupt
    }[event_type](callback)

def register_events(crawler, **subscriptions):
    for key in subscriptions:
        assert key in ['on_start', 'on_interrupt', 'on_finish'],\
                         "Unrecognised event type=[%s]" % key
        if isinstance(subscriptions[key], list):
            for sub in subscriptions[key]:
                register(crawler, key, sub)
        else:
            register(crawler, key, subscriptions[key])
