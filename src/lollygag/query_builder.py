from .query import compile_query


class QueryBuilder(object):
    def __init__(self, **kwargs):
        self.data = kwargs
        self.chain = 'or'

    def compile(self):
        return compile_query(self.data)

    def name(self, value):
        return self.__add_query({'name': value})

    def attribute(self, key, value):
        return self.__add_query({'attribute': (key, value)})

    def data(self, value):
        return self.__add_query({'data': value})

    def _or(self):
        self.chain = 'or'
        return self

    def _and(self):
        self.chain = 'and'
        return self

    def _not(self, query):
        return self.__add_query({'not': query})

    def __add_query(self, query):
        if self.data:
            self.data = {self.chain: (self.data, query)}
            self.chain = 'or'
        else:
            self.data = query
        return self
