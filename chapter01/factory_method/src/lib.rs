use std::fs;

use serde_json::Value;

pub struct JsonDataExtractor {
    parsed_data: Value,
}

pub struct XmlDataExtractor {}

impl JsonDataExtractor {
    pub fn new(data: &String) -> JsonDataExtractor {
        JsonDataExtractor {
            parsed_data: serde_json::from_str(data).unwrap(),
        }
    }

    pub fn parse_data(&self) -> &Value {
        &self.parsed_data
    }

    pub fn print_data(&self) {
        println!("{}",serde_json::to_string_pretty(&self.parsed_data).unwrap());
    }
}

impl XmlDataExtractor {}

fn data_extraction_factory_for_json(filepath: String) -> JsonDataExtractor {
    let data = get_string_from_file(filepath);

    JsonDataExtractor::new(&data)
}

pub fn extract_data_from_for_json(filepath: String) -> JsonDataExtractor {
    data_extraction_factory_for_json(filepath)
}

fn get_string_from_file(filepath: String) -> String {
    fs::read_to_string(filepath).expect("Something went wrong while reading the file")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
