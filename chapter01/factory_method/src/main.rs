use factory_method::extract_data_from_for_json;

fn main() {
    let json_factory = extract_data_from_for_json(String::from("data/movies.json"));
    let json_data = json_factory.parse_data();

    // println!("{:?}", json_data);
    json_factory.print_data();

    let xml_factory = extract_data_from_for_xml(String::from("data/person.xml"));
    // let xml_data = xml_factory.parse_data();

    xml_factory.print_data();
}
