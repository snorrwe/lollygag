#!/usr/bin/env python

from setuptools import setup, find_packages
try:
    from setuptools_rust import RustExtension, Binding
except ImportError:
    import sys
    import subprocess
    subprocess.check_call(
        [sys.executable, '-m', 'pip', 'install', 'setuptools_rust'])
    from setuptools_rust import RustExtension, Binding

DEPENDENCIES = ['requests>=2.2.1']
TEST_DEPENDENCIES = ['pytest']

if __name__ == '__main__':
    setup(
        name='lollygag',
        author='Daniel Kiss',
        version="2.0.0",
        author_email='littlesnorrboy@gmail.com',
        url='https://github.com/snorrwe/lollygag',
        description="A simple web crawling module",
        license="MIT",
        package_dir={'': 'src'},
        packages=find_packages('src'),
        install_requires=DEPENDENCIES,
        rust_extensions=[
            RustExtension(
                'lollygag_ext',
                'lollygag_ext/py_interface/Cargo.toml',
                binding=Binding.RustCPython)
        ],
        tests_require=TEST_DEPENDENCIES)
