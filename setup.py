#!/usr/bin/env python

import os
import re
from setuptools import setup, find_packages

try:
  import pandoc
  doc = pandoc.Document()
  with open("README.md") as file:
    doc.markdown = file.read()
    home_page = doc.rst
except:
  home_page = ""

try:
  tag = os.environ['TRAVIS_TAG']
  re.search(r'(\d\.){3,}', tag).group(0)
except:
  version = 'UNKNOWN_VERSION'

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