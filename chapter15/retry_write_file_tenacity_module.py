import time
import sys
import os
import tenacity


def create_file(filename, after_delay=5):
    time.sleep(after_delay)

    with open(filename, 'w') as f:
        f.write('A file creation test')

#@tenacity.retry(wait=tenacity.wait_fixed(2))
@tenacity.retry(wait=tenacity.wait_exponential())
def append_data_to_file(filename):

    if os.path.exists(filename):
        print("got the file... let's proceed!")
        with open(filename, 'a') as f:
            f.write(' ...Updating the file')
        return "OK"
    else:
        print("Error: Missing file, so we can't proceed. Retrying...")
        raise OSError

FILENAME = 'file2.txt'

if __name__ == "__main__":
    args = sys.argv

    if args[1] == 'create':
        create_file(FILENAME)
        print(f"Created file '{FILENAME}'")
    elif args[1] == 'update':
        while True:
            out = append_data_to_file(FILENAME)
            if out == "OK":
                print("Success! We are done!")
                break
