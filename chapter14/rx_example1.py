from rx import Observable, Observer

def get_quotes():
    import contextlib, io
    zen = io.StringIO()
    with contextlib.redirect_stdout(zen):
        import this

    quotes = zen.getvalue().split('\n')[1:]
    return quotes

def push_quotes(obs):

    quotes = get_quotes()
    for q in quotes:
        if q:  # skip empty
            obs.on_next(q)
    obs.on_completed()

class ZenQuotesObserver(Observer):

    def on_next(self, value):
        print(f"Received: {value}")

    def on_completed(self):
        print("Done!")

    def on_error(self, error):
        print(f"Error Occurred: {error}")

source = Observable.create(push_quotes)

source.subscribe(ZenQuotesObserver())
