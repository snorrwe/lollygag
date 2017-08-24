# Frenetiq Crawler

## About

* Version: 0.0.1
* Travis CI: [![Build Status](https://travis-ci.com/snorrwe/Crawler.svg?token=WMyzvqbod1Qz3yx7UKAT&branch=master)](https://travis-ci.com/snorrwe/Crawler)
* Supported Python versions: 
    * Python 2.7
    * Python 3.6
    * Pypy
    * Pypy 3

## Installation

`pip install frenetiq-crawler`

## Usage

### Example code

```python
#!/usr/bin/python

from frenetiq_crawler import run
from frenetiq_crawler.services import SERVICES
from frenetiq_crawler.crawler.link_crawler import LinkCrawler

class MyCrawler(LinkCrawler):
    # This is a custom crawler to handle a single webpage
    # If you wish to preserve existing functionality when overriding methods,
    # do not forget to call the super method aswell
    def feed(self, data):
        # Override the feed method to customize handling the whole page data
        self.log_service.info("Yeah boi, a page!")
        return LinkCrawler.feed(self, data)

    def handle_starttag(self, tag, attrs):
        # Override HTMLParser methods for custom behaviour
        # Call the super method to return links on the page
        # The default DomainCrawler class will continue crawling if crawl() returns a 
        # CrawlResult with links
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
```

```bash
$ python crawler_example.py -u snorrwe.github.io/crawler_test

[Info]Thread=[MainThread]        ----------Crawl starting----------
[Debug]Thread=[MainThread]       No urls to crawl, going to sleep. Work in progress=[1]
[Info]Thread=[WSc--3]    Yeah boi, a page!
[Debug]Thread=[WSc--3]   Boi, I found a link!
[Debug]Thread=[WSc--3]   Boi, I found a link!
[Info]Thread=[WSc--3]    Yeeeeaaaah boiiiiiii, I found an img
[Info]Thread=[WSc--3]    Link=[http://snorrwe.github.io/crawler_test] StatusCode=[200] Size=[310]
[Debug]Thread=[WSc--3]   URLs visited=[1], remaining=[2]
[Debug]Thread=[MainThread]       No urls to crawl, going to sleep. Work in progress=[2]
[Info]Thread=[WSc--8]    Yeah boi, a page!
[Debug]Thread=[WSc--8]   Boi, I found a link!
[Info]Thread=[WSc--8]    Link=[http://snorrwe.github.io/crawler_test/kanga.html] StatusCode=[200] Size=[220]
[Debug]Thread=[WSc--8]   URLs visited=[2], remaining=[0]
[Info]Thread=[WSc--7]    Link=[http://snorrwe.github.io/crawler_test/kanga2.html] StatusCode=[404] Size=[9340]
[Debug]Thread=[WSc--7]   URLs visited=[3], remaining=[0]
[Info]Thread=[MainThread]        Total urls crawled=[3] Urls left to crawl=[0]
[Info]Thread=[MainThread]        ----------Crawl finished----------
```

### Command line arguments

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Short</th>
            <th>Description</th>
        </tr>
    </thead>
    <tbody>
        <div>
            <tr>
                <td>--help</td>
                <td>-h</td>
                <td rowspan="2">Show the help and exit</td>
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
                    Note that if you pass the url argument to run() or crawl() this option will be ignored.
                </i>
                </td>
            </tr>
            <tr>
            </tr>
        </div>
        <div>
            <tr>
                <td>--threads</td>
                <td>-t</td>
                <td rowspan="2">Maximum number of concurrent threads</td>
            </tr>
            <tr>
            </tr>
        </div>
        <div>
            <tr>
                <td>--loglevel</td>
                <td>-l</td>
                <td rowspan="2">Level of logging, possible values = [all, info, debug, warn, error, none]</td>
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

Test files are located in the _e2e_ directory.<br>

## License

[MIT](https://github.com/snorrwe/Crawler/blob/master/LICENSE)
