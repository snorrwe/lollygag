import json
import os
import sys

here = os.path.dirname(os.path.realpath(__file__))
version_file = os.path.join(here, "lollygag", ".version")

def main():
    try:
        with open(version_file, 'r') as infile:
            metadata = json.loads(infile.read())
        if "TRAVIS_TAG" in os.environ and os.environ["TRAVIS_TAG"] != metadata["version"]:
            metadata["version"] = os.environ["TRAVIS_TAG"]
            with open(version_file, 'w') as outfile:
                outfile.write(json.dumps(metadata))
    except:
        print("Unexpected error while setting up version!")
        error = sys.exc_info()
        print(error[0])
        print(error[1])
        print(str(error[2]))
        sys.exit(1)

if __name__ == '__main__':
    main()
