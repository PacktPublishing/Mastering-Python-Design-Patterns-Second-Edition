use std::io;
use std::num::ParseIntError;

struct QuoteModel {
    db: Vec<String>,
}

struct QuoteTerminalView {}

pub struct QuoteTerminalController {
    model: QuoteModel,
    view: QuoteTerminalView,
}

impl QuoteModel {
    fn new(db: Vec<String>) -> QuoteModel {
        QuoteModel { db: db }
    }

    fn get_quote(&self, n: usize) -> Option<&String> {
        self.db.get(n)
    }
}

impl QuoteTerminalView {
    fn new() -> QuoteTerminalView {
        QuoteTerminalView {}
    }

    fn show(&self, quote: &String) {
        println!("And the quote is: {}", quote);
    }

    fn error(&self, msg: &String) {
        println!("Error: {}", msg);
    }

    fn select_quote(&self) -> Result<usize, ParseIntError> {
        let mut selection = String::new();

        println!("Which quote number would you like to see? ");
        io::stdin().read_line(&mut selection);

        selection.trim().parse()
    }
}

impl QuoteTerminalController {
    pub fn new(db: Vec<String>) -> QuoteTerminalController {
        QuoteTerminalController {
            model: QuoteModel::new(db),
            view: QuoteTerminalView::new(),
        }
    }

    pub fn run(&self) {
        let mut valid_input = false;

        while !valid_input {
            match self.view.select_quote() {
                Ok(idx) => { 
                    match self.model.get_quote(idx) {
                        Some(quote) => { 
                            valid_input = true;
                            self.view.show(quote);
                        },
                        None => self.view.show(&String::from("Not Found!")),
                    }
                },
                Err(_) => self.view.error(&String::from("Incorrect index")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model() {
        let mut quote_db = Vec::new();

        quote_db.push(String::from("test quote 1"));
        quote_db.push(String::from("test quote 2"));

        let model = QuoteModel::new(quote_db);

        assert_eq!(model.get_quote(0).unwrap(), "test quote 1");
        assert_eq!(model.get_quote(1).unwrap(), "test quote 2");
        assert_eq!(model.get_quote(2), None);
    }
}
