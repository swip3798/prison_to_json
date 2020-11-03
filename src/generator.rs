extern crate json;

struct Generator {
    prison_json: json::JsonValue,
    cursor: usize
}

// Static 
impl Generator {
    fn new(prison_json: json::JsonValue) -> Generator {
        Generator {
            prison_json,
            cursor: 0
        }
    }
}

// Instance
impl Generator {
    fn compile(&mut self) -> Vec<String> {
        let mut tokens = Vec::<String>::new();
        

        return tokens;
    }
}