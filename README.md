# Lollygag

## About

* Travis CI: [![Build Status](https://travis-ci.org/snorrwe/lollygag.svg?branch=master)](https://travis-ci.org/snorrwe/lollygag)
* Supported Python versions: 
    * Python 2.7
    * Python 3.6
    * Pypy
    * Pypy 3

## Installation

`pip install lollygag`

## Usage

### Sample code

```python
#!/usr/bin/python

from lollygag import run
from lollygag.services import Services
from lollygag.core.parsers.link_parser import LinkParser

# Override HTMLParser methods to provide a custom implementation
# https://docs.python.org/2/library/htmlparser.html
# Be sure to call the super methods so you do not lose existing functionality!
#
# Or use a different parser like Beautiful Soup
class MyCrawler(LinkParser):
    def feed(self, data):
        self.log_service.info("Yeah boi, a page!")
        return super(MyCrawler, self).feed(data)

    def handle_starttag(self, tag, attrs):
        super(MyCrawler, self).handle_starttag(tag, attrs)
        if(tag == 'img'):
            self.log_service.info("Yeeeeaaaah boiiiiiii, I found an %s" % (tag))

def main():
    # Override site_crawler_factory with my own implementation
    Services.site_crawler_factory = MyCrawler
    run()

if __name__ == '__main__':
    main()

```

```bash
python crawler_example.py -u snorrwe.github.io/crawler_test

[Info]Thread=[MainThread]        ----------Crawl starting----------
[Debug]Thread=[MainThread]       No urls to crawl, going to sleep. Work in progress=[1]
[Info]Thread=[WSc--0]    Yeah boi, a page!
[Info]Thread=[WSc--0]    Yeeeeaaaah boiiiiiii, I found an img
[Info]Thread=[WSc--0]    Link=[http://snorrwe.github.io/crawler_test] StatusCode=[200] Size=[310]
[Debug]Thread=[WSc--0]
    Urls visited=[1]
    Urls in progess=[0]
    Urls left=[2]
[Debug]Thread=[MainThread]       No urls to crawl, going to sleep. Work in progress=[2]
[Info]Thread=[WSc--3]    Link=[http://snorrwe.github.io/crawler_test/kanga2.html] StatusCode=[404] Size=[9340]
[Debug]Thread=[WSc--3]
    Urls visited=[2]
    Urls in progess=[1]
    Urls left=[0]
[Info]Thread=[WSc--4]    Yeah boi, a page!
[Info]Thread=[WSc--4]    Link=[http://snorrwe.github.io/crawler_test/kanga.html] StatusCode=[200] Size=[220]
[Debug]Thread=[WSc--4]
    Urls visited=[3]
    Urls in progess=[0]
    Urls left=[0]
[Info]Thread=[MainThread]        -------------Yeah boiiii, done-----------------
[Info]Thread=[MainThread]        --------------------Crawl status--------------------
                                        Urls visited=[3]
                                        Urls in progess=[0]
                                        Urls left=[0]
[Info]Thread=[MainThread]        ----------Crawl finished----------
```

### Command line arguments

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Short</th>
            <th>Description</th>
            <th>Default</th>
        </tr>
    </thead>
    <tbody>
        <div>
            <tr>
                <td>--help</td>
                <td>-h</td>
                <td rowspan="2">Show the help and exit</td>
                <td> - </td>
            </tr>
            <tr>
            </tr>
        </div>
        <div>
            <tr>
                <td>--url</td>
                <td>-u</td>
                <td rowspan="2">Base url you wish to crawl.<br>
                <i>
                    Note that if you pass the url argument to run() or crawl_domain() this option will be ignored.
                </i>
                </td>
                <td> - </td>
            </tr>
            <tr>
            </tr>
        </div>
        <div>
            <tr>
                <td>--threads</td>
                <td>-t</td>
                <td rowspan="2">Maximum number of concurrent threads</td>
                <td> 5 </td>
            </tr>
            <tr>
            </tr>
        </div>
        <div>
            <tr>
                <td>--loglevel</td>
                <td>-l</td>
                <td rowspan="2">Level of logging, possible values = [all, debug, info, warn, error, none]</td>
                <td> all </td>
            </tr>
            <tr>
            </tr>
        </div>
    </tbody>
</table>

## Testing

### Running the unit tests

Running the unit tests with the __pytest__ package.<br>
To install pytest via pip run `pip install pytest`.

To run the tests run `pytest` in the root folder.

### Test code

#### Unit testing

Test files are located next to the files they meant to test.<br>
Test files have the same base name, and end with *_test*<br>
For example: tests for file *some_source.py* are in the file *some_source_test.py*

#### End-to-end testing

Test files are located in the _test_ directory.<br>

## License

[MIT](https://github.com/snorrwe/Crawler/blob/master/LICENSE)
