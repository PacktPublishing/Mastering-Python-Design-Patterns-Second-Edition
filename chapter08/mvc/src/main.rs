use mvc::QuoteTerminalController;

fn main() {
    let mut quote_db = Vec::new();

    quote_db.push(String::from("A man is not complete until he is married. Then he is finished."));
    quote_db.push(String::from("As I said before, I never repeat myself."));
    quote_db.push(String::from("Behind a successful man is an exhausted woman."));
    quote_db.push(String::from("Black holes really suck..."));
    quote_db.push(String::from("Facts are stubborn things."));

    let controller = QuoteTerminalController::new(quote_db);

    while true {
        controller.run();
    }
}
