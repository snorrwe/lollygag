#!/usr/bin/env python

import os
import re
import sys
from setuptools import setup, find_packages

config_file = "setup.cfg"
home_page = """
Lollygag
========

About
-----

See: https://github.com/snorrwe/lollygag

Supported Python versions:
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

def create_version_file(version):
    with open(config_file, 'w') as f:
        conf = """[metadata]
version={version}
        """.format(version=version)
        f.write(conf)

def main():
    global home_page

    try:
        version = os.environ['TRAVIS_TAG']
        if version:
            create_version_file(version)
    except (KeyError, IOError):
        pass
    except:
        handle_possible_ci_error("Something went wrong while setting the version!", 2)

    setup(name='lollygag',
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
        }
       )

if __name__ == '__main__':
  main()
