#!/usr/bin/env python

from setuptools import setup, find_packages

try:
  import markdown
  markdown.markdownFromFile(input="README.md", output="README.html")
  with open("README.html") as file:
    home_page = file.read()
except:
  home_page = ""

setup(name='frenetiq_crawler',
      version='0.0.2',
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