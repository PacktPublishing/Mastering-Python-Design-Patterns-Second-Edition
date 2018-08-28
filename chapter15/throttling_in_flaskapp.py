from flask import Flask
from flask_limiter import Limiter
from flask_limiter.util import get_remote_address

from flask import Flask
app = Flask(__name__)
limiter = Limiter(
    app,
    key_func=get_remote_address,
    default_limits=["100 per day", "10 per hour"]
)

@app.route("/limited")
def limited_api():
    return "Welcome to our API!"

@app.route("/more_limited")
@limiter.limit("2/minute")
def more_limited_api():
    return "Welcome to our expensive, thus very limited, API!"


if __name__ == "__main__":
    app.run(debug=True)
