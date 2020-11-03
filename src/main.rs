mod lexer;
mod filereader;
mod parser;
mod filewriter;
mod generator;

#[allow(dead_code)]
fn prison_to_json() -> json::JsonValue {
    let reader = filereader::FileReader::new(String::from("example.prison"));
    let raw_prison = reader.to_string_with_default(String::from("lol"));
    let tokens = lexer::tokenize(raw_prison);
    let mut parser = parser::Parser::new(tokens);
    let prison_obj = parser.get_json_value();
    let writer = filewriter::FileWriter::new(String::from("output.json"));
    writer.write_or_update(prison_obj.pretty(2));
    return prison_obj;
}

#[allow(dead_code)]
fn json_to_prison(obj: json::JsonValue) {
    let mut gen = generator::Generator::new(obj);
    let tokens = gen.generate_prison();
    let writer = filewriter::FileWriter::new(String::from("output.prison"));
    writer.write_or_update(tokens.join(""));
}

fn main() {
    println!("Parsing prison file");
    let obj = prison_to_json();
    println!("Generating prison file");
    json_to_prison(obj);
}