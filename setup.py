#!/usr/bin/env python

import os
import re
import sys
from setuptools import setup, find_packages

def handle_possible_ci_error(message, code):
    print(message)
    is_deploy = False
    try:
        is_deploy = os.environ['SETUP_DEPLOY']
    except KeyError:
        pass
    if is_deploy:
        print(sys.exc_info())
        raise SystemExit(code)

def main():
  try:
    import pypandoc
    home_page = pypandoc.convert_file("README.md", 'rst')
    assert home_page
  except:
    handle_possible_ci_error("Something went wrong while generating the README!", 1)
    home_page = "Something went wrong while generating the README. Please refer to https://github.com/snorrwe/frenetiq-crawler"

  try:
    version = os.environ['TRAVIS_TAG']
    re.search(r'(\d\.){3,}', version).group(0)
  except:
    handle_possible_ci_error("Something went wrong while setting the version!", 2)
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
