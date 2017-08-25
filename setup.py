#!/usr/bin/env python

import os
import re
import sys
from setuptools import setup, find_packages

def main():
  try:
    import pandoc
    doc = pandoc.Document()
    with open("README.md") as file:
      doc.markdown = file.read()
      home_page = doc.rst
  except:
    print("Something went wrong while generating the README!")
    home_page = "Something went wrong while generating the README. Please refer to https://github.com/snorrwe/frenetiq-crawler"

  try:
    tag = os.environ['TRAVIS_TAG']
    re.search(r'(\d\.){3,}', tag).group(0)
  except:
    version = '0.0.dev1'

  setup(name='frenetiq_crawler',
        version=version,
        author='Daniel Kiss',
        author_email='littlesnorrboy@gmail.com',
        url='https://github.com/snorrwe/frenetiq-crawler',
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
