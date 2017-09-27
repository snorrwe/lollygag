#!/usr/bin/python

from unittest import TestCase, main as unittest_main
import subprocess
import re
import sys
import os

URI = "https://snorrwe.github.io/crawler_test/"
HERE = os.path.realpath(os.path.abspath(__file__))
HERE = os.path.dirname(HERE)

IS_WINDOWS = sys.platform.startswith("win")

EXPECTED = """[Info]Thread=[MainThread]        ----------Crawl starting----------
[Debug]Thread=[MainThread]       No urls to crawl, going to sleep. Work in progress=[1]
[Info]Thread=[WSc--0]    Yeah boi, a page!
[Debug]Thread=[WSc--0]   Boi, I found an anchor tag!
[Debug]Thread=[WSc--0]   Boi, I found an anchor tag!
[Info]Thread=[WSc--0]    Yeeeeaaaah, I found an img
[Info]Thread=[WSc--0]    Link=[https://snorrwe.github.io/crawler_test/] StatusCode=[200] Size=[310]
[Debug]Thread=[WSc--0]   --------------------Crawl status--------------------
                                        Urls visited=[1]
                                        Urls in progess=[0]
                                        Urls left=[2]
[Info]Thread=[WSc--0]    Link=[https://snorrwe.github.io/crawler_test/kanga2.html] StatusCode=[404] Size=[9340]
[Debug]Thread=[WSc--0]   --------------------Crawl status--------------------
                                        Urls visited=[2]
                                        Urls in progess=[0]
                                        Urls left=[1]
[Info]Thread=[WSc--0]    Yeah boi, a page!
[Debug]Thread=[WSc--0]   Boi, I found an anchor tag!
[Info]Thread=[WSc--0]    Link=[https://snorrwe.github.io/crawler_test/kanga.html] StatusCode=[200] Size=[220]
[Debug]Thread=[WSc--0]   --------------------Crawl status--------------------
                                        Urls visited=[3]
                                        Urls in progess=[0]
                                        Urls left=[0]
[Info]Thread=[MainThread]        -------------Yeah boiiii, done-----------------
[Info]Thread=[MainThread]        --------------------Crawl status--------------------
                                        Urls visited=[3]
                                        Urls in progess=[0]
                                        Urls left=[0]
[Info]Thread=[MainThread]        ----------Crawl finished----------"""


class BasicTest(TestCase):
    output = None
    lines = None

    @classmethod
    def setUpClass(cls):
        commands = [os.path.join(HERE, "crawler_example.py"), "-u", URI]
        print("Commands:", commands)
        crawler_process = subprocess.Popen(commands
            , stdout=subprocess.PIPE, shell=IS_WINDOWS)
        (output, error) = crawler_process.communicate()
        print(output, error)
        cls.output = output.decode("utf-8") if output else ""
        lines = cls.output.splitlines()
        cls.lines = [i for i in lines if i != '']

    def test_test_sanity(self):
        self.assertTrue(self.output)
        self.assertTrue(self.lines)
        try:
            self.assertTrue(type(self.output) is unicode, "Output should be of type unicode, instead got %s" % (type(self.output)))
        except NameError: # python 3 support
            self.assertTrue(type(self.output) is str, "Output should be of type str, instead got %s" % (type(self.output)))
        expected = len(EXPECTED.splitlines())
        self.assertTrue(len(self.lines) == expected, "Output should contain %s lines, instead got %s" % (expected, len(self.lines)))

    def test_found_all_pages(self):
        found = []
        for line in self.lines:
            if re.search(r'Link=\[.+\] StatusCode=\[\d+\] Size=\[\d+\]', line):
                found.append(re.search(r'Link=\[.+\]', line).group(0))
        self.assertEqual(len(found), 3)
        self.assertEqual(len(found), len(set(found)))

    def test_displayed_correct_results(self):
        page_data = {}
        for line in self.lines:
            output = re.search(r'Link=\[(?P<link>.+)\] StatusCode=\[(?P<code>\d+)\] Size=\[\d+\]', line)
            if(output):
                link = output.group('link')
                code = output.group('code')
                page_data[link] = code
        self.assertTrue('%s' % (URI)in page_data)
        self.assertTrue('%skanga.html' % (URI) in page_data)
        self.assertTrue('%skanga2.html' % (URI) in page_data)
        self.assertEqual(page_data['%s' % (URI)], '200')
        self.assertEqual(page_data['%skanga.html' % (URI)], '200')
        self.assertEqual(page_data['%skanga2.html' % (URI)], '404')

    def test_got_what_was_expected(self):
        for line in self.lines:
            isExpected = EXPECTED.find(line)
            self.assertTrue(isExpected, "Line=[{line}] was unexpected!".format(line=line))

if __name__ == '__main__':
    unittest_main()
