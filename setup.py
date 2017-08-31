#!/usr/bin/env python

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

def main():
    global home_page

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
