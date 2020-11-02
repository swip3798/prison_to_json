mod lexer;
mod filereader;
mod parser;
mod filewriter;
mod compiler;

#[allow(dead_code)]
fn prison_to_json() {
    let reader = filereader::FileReader::new(String::from("example.prison"));
    let raw_prison = reader.to_string_with_default(String::from("lol"));
    let tokens = lexer::tokenize(raw_prison);
    let mut parser = parser::Parser::new(tokens);
    let prison_obj = parser.get_json_value();
    let writer = filewriter::FileWriter::new(String::from("output.json"));
    writer.write_or_update(prison_obj.pretty(2));
}

#[allow(dead_code)]
fn json_to_prison() {

}

fn main() {
    //prison_to_json();
    json_to_prison();
}