extern crate json;

#[derive(Debug)]
pub enum Error {
    UnexpectedTypeInArray,
    UnexpectedType
}

pub struct Generator {
    prison_json: json::JsonValue,
}

// Static 
impl Generator {
    pub fn new(prison_json: json::JsonValue) -> Generator {
        Generator {
            prison_json
        }
    }

    fn check_token(token: String) -> String {
        if token.contains(" ") {
            return format!("\"{}\"", token);
        } else {
            return token;
        }
    }
    fn generate_pair(key: &str, value: String) -> Vec<String> {
        let mut tokens = Vec::<String>::new();
        if key.contains(" ") {
            tokens.push(format!("\"{}\"", key));
        } else {
            tokens.push(Generator::check_token(key.to_string()));
        }
        tokens.push(String::from(" "));
        tokens.push(Generator::check_token(value));
        return tokens;
    }

    fn generate_obj(key: &str, obj: &json::JsonValue, indent: u16) -> Result<Vec<String>, Error> {
        let mut tokens = Vec::<String>::new();
        tokens.push(String::from("BEGIN"));
        tokens.push(String::from(" "));
        tokens.push(Generator::check_token(key.to_string()));
        tokens.push(String::from("\n"));
        tokens.append(&mut Generator::generate(obj, indent)?);
        for _ in 0..(indent - 1) {
            tokens.push(String::from("    "));
        }
        tokens.push(String::from("END"));
        return Ok(tokens);
    }

    fn generate(obj: &json::JsonValue, indent: u16) -> Result<Vec<String>, Error> {
        let mut tokens = Vec::<String>::new();
       
        for element in obj.entries() {
            for _ in 0..indent {
                tokens.push(String::from("    "));
            }
            if element.1.is_string() {
                // Unwrap acceptable cause if clause already checks if element is string
                tokens.append(&mut Generator::generate_pair(element.0, element.1.as_str().unwrap().to_string()));
            } else if element.1.is_array() {
                for member in element.1.members() {
                    if member.is_string() {
                        tokens.append(&mut Generator::generate_pair(element.0, member.as_str().unwrap().to_string()));
                    } else if member.is_object() {
                        tokens.append(&mut Generator::generate_obj(element.0, member, indent + 1)?);
                    } else {
                        return Err(Error::UnexpectedTypeInArray);
                    }
                }
            } else if element.1.is_object() {
                tokens.append(&mut Generator::generate_obj(element.0, element.1, indent + 1)?);
            } else {
                return Err(Error::UnexpectedType);
            }
            tokens.push(String::from("\n"));
        }
        return Ok(tokens);
    }
}

// Instance
impl Generator {
    pub fn generate_prison(&mut self) -> Result<Vec<String>, Error> {
        return Generator::generate(&self.prison_json, 0);
    }
}