# Contributing to lollygag

### Did you find a bug?

* **Ensure the bug was not already reported** by searching on GitHub under [Issues](https://github.com/snorrwe/lollygag/issues).

* If you're unable to find an open issue addressing the problem, [open a new one](https://github.com/snorrwe/lollygag/issues/new). Be sure to include a **title and clear description**, as much relevant information as possible, and a **code sample** or an **executable test case** demonstrating the expected behavior that is not occurring.

### Did you write a patch that fixes a bug?

* Open a new GitHub pull request with the patch.

* Ensure the PR description clearly describes the problem and solution. Include the relevant issue number if applicable.

* Pull requests must pass all tests on Travis CI and must be peer reviewed by a contributor before they can be merged into _master_. To run the tests locally:
    * Run the tests via (for example) _pytest_. Just run `pytest` in the root directory of the project.
    * Run _pylint_. Run `pylint lollygag` in the root directory of the project.
    
### Did you fix whitespace, format code, or make a purely cosmetic patch?

We accept purely cosmetic fixes aswell. Submit a pull request!

### Do you intend to add a new feature or change an existing one?

* Open a new branch for your feature.

* Open a pull request once you're finished.

* Prefer many smaller commits to fewer larger ones.

* If you are unsure about the feature open a new issue with the _question_ label and wait for feedback.

### Coding guidelines

* Your code must pass the _lint_ checks.

* Write at least unit tests for your code.

* Please see the __Test code__ section of this document about test code.

* Write docstrings for your classes and public methods

### Test code

#### Unit testing

Test files are located next to the files they meant to test.<br>
Test files have the same base name, and end with *_test*<br>
For example: tests for file *some_source.py* are in the file *some_source_test.py*

#### End-to-end testing

Test files are located in the *black_box_tests* directory.<br>

### Do you have questions about the source code?

Either open a new issue with the _question_ label. Or you can contact **littlesnorrboy@gmail.com** directly.

Cheers:

[Daniel](https://github.com/snorrwe)
