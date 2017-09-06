from lollygag.core.domain_crawler import DomainCrawler

def make_node(lst):
    if isinstance(lst, dict):
        return lst
    result = dict()
    for item in lst:
        result[item] = {}
    return result

class MapperCrawler(DomainCrawler):

    def __init__(self, *args, **kwargs):
        super(MapperCrawler, self).__init__(*args, **kwargs)
        self.graph = set()

    def reset(self, url):
        super(MapperCrawler, self).reset(url)
        self.graph = set()

    def process_links(self, origin, links):
        result = super(MapperCrawler, self).process_links(origin, links)
        for link in result:
            self.graph.add((origin, link))
        return result

    def make_map(self):
        result = self.__make_map()
        return result

    def __make_map(self):
        result = {}
        for vertex in self.graph:
            for value in vertex:
                if value not in result:
                    result[value] = []
            result[vertex[0]].append(vertex[1])
        return result
