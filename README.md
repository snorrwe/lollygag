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
    # Override site_parser_factory with my own implementation
    Services.site_parser_factory = MyCrawler
    run()

if __name__ == '__main__':
    main()

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

## Contributing

Please refer to the [contribution guidelines](https://github.com/snorrwe/lollygag/blob/master/.github/CONTRIBUTING.md)

## License

[MIT](https://github.com/snorrwe/Crawler/blob/master/LICENSE)
