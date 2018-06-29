from .query import compile_query


class QueryBuilder(object):
    def __init__(self, **kwargs):
        self.data = kwargs

    def compile(self):
        return compile_query(self.data)

    def name(self, value):
        return self.__add_query({'name': value})

    def attribute(self, key, value):
        return self.__add_query({'attribute': (key, value)})

    def data(self, value):
        return self.__add_query({'data': value})

    #  def or(self, query):
    #      return

    def __add_query(self, query):
        if self.data:
            self.data = {'or': (self.data, query)}
        else:
            self.data = query
        return self
