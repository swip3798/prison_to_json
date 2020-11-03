mod lexer;
mod filereader;
mod parser;
mod filewriter;
mod generator;

extern crate clap;
use clap::{App, Arg}; 
use json;

fn main() {
    let matches = App::new("prison_to_json")
       .version("v1.0.0")
       .about("Can parse Prison Architect savefiles to json and back.")
       .author("Created by: Christian Schweigel")
       .arg(Arg::with_name("INPUTFILE")
                               .help("Input file, can be PA savefile or json")
                               .required(true)
                               .index(1))
       .get_matches();
    let input_file = matches.value_of("INPUTFILE").unwrap();
    let is_json = &input_file[input_file.len()-4..input_file.len()] == "json";
    let is_prison = &input_file[input_file.len()-6..input_file.len()] == "prison";
    let reader = filereader::FileReader::new(input_file.to_string());
    let result = reader.to_string();
    match result {
        Some(content) => {
            if is_json {
                let output_file = input_file[..input_file.len()-5].to_string() + ".prison";
                let parse = json::parse(content.as_str());
                match parse {
                    Ok(obj) => {
                        let mut gen = generator::Generator::new(obj);
                        let tokens = gen.generate_prison().unwrap();
                        let writer = filewriter::FileWriter::new(output_file.clone());
                        writer.write_or_update(tokens.join(""));
                        println!("Json file parsed and prison savefile generated: {}", output_file);
                    },
                    _ => {
                        println!("Json file could not be parsed!");
                    }
                }
            } else if is_prison {
                let tokens = lexer::tokenize(content);
                let mut parser = parser::Parser::new(tokens);
                let prison_obj = parser.get_json_value();
                let output_file = input_file[..input_file.len()-7].to_string() + ".json";
                let writer = filewriter::FileWriter::new(output_file.clone());
                writer.write_or_update(prison_obj.pretty(2));
                println!("Prison savefile parsed and converted to json file: {}", output_file);
            } else {
                println!("Invalid file extension!");
            }
        }
        _ => {
            println!("The given filename could not be found!");
        }
    }
}