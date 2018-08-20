
# our version of a script provided in https://github.com/veltra/pybreaker-playground

import pybreaker
from datetime import datetime
import random
from time import sleep


breaker = pybreaker.CircuitBreaker(fail_max=2, reset_timeout=5)

@breaker
def fragile_function():
    if not random.choice([True, False]):
        print(' / OK', end='')
    else:
        print(' / FAIL', end='')
        raise Exception('This is a sample Exception')


if __name__ == "__main__":

    while True:
        print(datetime.now().strftime('%Y-%m-%d %H:%M:%S'), end='')

        try:
            fragile_function()
        except Exception as e:
            print(' / {} {}'.format(type(e), e), end='')
        finally:
            print('')
            sleep(1)
