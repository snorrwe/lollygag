#!/usr/bin/env python

import os
import re
import sys
from setuptools import setup, find_packages

home_page = """
Lollygag
========

About
-----

-  See: https://github.com/snorrwe/lollygag
-  Supported Python versions:
   -  Python 2.7
   -  Python 3.6
   -  Pypy
   -  Pypy 3

Installation
------------

``pip install lollygag``
"""

def handle_possible_ci_error(message, code):
    print(message)
    print(sys.exc_info())
    is_deploy = False
    try:
        is_deploy = os.environ['SETUP_DEPLOY']
    except KeyError:
        pass
    if is_deploy:
        raise SystemExit(code)

def main():
    global home_page

    version = '0.0.dev1'
    try:
        version = os.environ['TRAVIS_TAG']
    except KeyError:
        pass
    except:
        handle_possible_ci_error("Something went wrong while setting the version!", 2)

    setup(name='lollygag',
        version=version,
        author='Daniel Kiss',
        author_email='littlesnorrboy@gmail.com',
        url='https://github.com/snorrwe/lollygag',
        description="A simple web crawling module",
        long_description=home_page,
        license="MIT",
        package_dir={'':'src'},
        packages=find_packages('src'),
        install_requires=['requests>=2.2.1'],
        entry_points={
            'console_scripts': [
                'sample=sample:main',
            ],
        },
        python_requires='>=2.7, >=3.4'
       )

if __name__ == '__main__':
  main()
