use factory_method::extract_data_from_for_json;

fn main() {
    let json_factory = extract_data_from_for_json(String::from("data/movies.json"));
    let json_data = json_factory.parse_data();

    println!("Found: {}", json_data.as_array().unwrap().len());
    for movie in json_data.as_array().unwrap() {
        println!("Title: {}", movie["title"]);
        if !movie["year"].is_null() { println!("Year: {}", movie["year"]) };
        if !movie["director"].is_null() { println!("Year: {}", movie["director"]) };
        if !movie["genre"].is_null() { println!("Year: {}", movie["genre"]) };
        println!("");
    }
}
