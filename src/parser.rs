extern crate json;

pub struct Parser {
    tokens: Vec<String>,
    cursor: usize
}

// Static
impl Parser {
    pub fn new(tokens: Vec<String>) -> Parser {
        Parser {
            tokens,
            cursor: 0
        }
    }
}

// Instance
impl Parser {
    fn look_ahead(&self, offset: usize) -> &String {
        self.tokens.get(self.cursor + offset).unwrap()
    }

    fn consume(&mut self) -> String {
        let consumed = self.tokens.get(self.cursor).unwrap();
        self.cursor += 1;
        return consumed.clone();
    }

    fn expect_obj(&mut self) ->json::JsonValue {
        let mut prison = json::JsonValue::new_object();
        while self.look_ahead(0) != "END" {
            if self.look_ahead(0) == "BEGIN" {
                self.consume();
                let key = self.consume();
                let value = self.expect_obj();
                if prison.has_key(key.as_str()) {
                    if prison[key.as_str()].is_array() {
                        prison[key.as_str()].push(value).unwrap();
                    } else {
                        prison.insert(key.as_str(), json::array![prison[key.as_str()].clone(), value]).unwrap();
                    }
                } else {
                    prison.insert(key.as_str(), value).unwrap();
                }
            } else {
                let (key, value) = self.expect_pair();
                //println!("{}, {}", key, value);
                if prison.has_key(key.as_str()) {
                    if prison[key.as_str()].is_array() {
                        prison[key.as_str()].push(value).unwrap();
                    } else {
                        prison.insert(key.as_str(), json::array![prison[key.as_str()].clone(), value]).unwrap();
                    }
                } else {
                    prison.insert(key.as_str(), value).unwrap();
                }
            }
        }
        self.consume();
        return prison;
    }

    fn expect_pair(&mut self) -> (String, String) {
        let key = self.expect_key();
        let value = self.expect_value();
        return (key, value);
    }

    fn expect_key(&mut self) -> String {
        return self.consume();
    }

    fn expect_value(&mut self) -> String {
        return self.consume();
    }

    fn parse(&mut self) -> json::JsonValue {
        self.cursor = 0;
        self.tokens.push(String::from("END"));
        let prison = self.expect_obj();
        return prison;
    }

    pub fn get_json_value(&mut self) -> json::JsonValue {
        return self.parse();
    }
}