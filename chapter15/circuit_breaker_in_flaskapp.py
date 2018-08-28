import pybreaker
import random

from flask import Flask
app = Flask(__name__)

breaker = pybreaker.CircuitBreaker(fail_max=2, reset_timeout=5)

@breaker
def fragile_function():
    if not random.choice([True, False]):
        print(' / OK', end='')
    else:
        print(' / FAIL', end='')
        raise Exception('This is a sample Exception')

@app.route("/")
def hello():

    fragile_function()
    return "Hello World"

if __name__ == "__main__":
    app.run(debug=True)
