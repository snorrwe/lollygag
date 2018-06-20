#!/usr/bin/env python

from setuptools import setup, find_packages
try:
    from setuptools_rust import RustExtension, Binding
except ImportError:
    import pip
    pip.main(["install", "setuptools_rust"])

DEPENDENCIES = ['requests>=2.2.1']
TEST_DEPENDENCIES = ['tl.testing==0.5', 'pylint==1.7.2']

if __name__ == '__main__':
    setup(
        name='lollygag',
        author='Daniel Kiss',
        version="0.2.0",
        author_email='littlesnorrboy@gmail.com',
        url='https://github.com/snorrwe/lollygag',
        description="A simple web crawling module",
        license="MIT",
        package_dir={'': 'src'},
        packages=find_packages(),
        install_requires=DEPENDENCIES,
        rust_extensions=[
            RustExtension(
                'lollygag_ext',
                'lollygag_ext/Cargo.toml',
                binding=Binding.RustCPython)
        ],
        tests_require=TEST_DEPENDENCIES)
