from lollygag.core.domain_crawler import DomainCrawler

def make_node(value, children = None):
    return {
        'value': value,
        'children': children if children else []
    }

class MapperCrawler(DomainCrawler):

    def __init__(self, *args, **kwargs):
        super(MapperCrawler, self).__init__(*args, **kwargs)
        self.tree = {}
        self.root = None

    def reset(self, url):
        super(MapperCrawler, self).reset(url)
        self.tree[url] = {
            'parent': None
            , 'children': None
        }
        self.root = url

    def crawl_site(self, url):
        result = super(MapperCrawler, self).crawl_site(url)
        self.tree[url]['children'] = result.links
        for link in result.links:
            self.tree[link] = {
                'parent': url
                , 'children': None
            }
        self.log_service.important("!!!!!!!!!!", self.make_map())
        return result

    def make_map(self):
        return self.__make_map(self.root)

    def __make_map(self, current):
        result = make_node(current)
        for child in self.tree[current]:
            result['children'].append(self.__make_map(child))
        return result
