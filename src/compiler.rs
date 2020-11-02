extern crate json;

struct Compiler {
    prison_json: json::JsonValue,
    cursor: usize
}

// Static 
impl Compiler {
    fn new(prison_json: json::JsonValue) -> Compiler {
        Compiler {
            prison_json,
            cursor: 0
        }
    }
}

// Instance
impl Compiler {
    fn compile(&mut self) -> Vec<String> {
        let mut tokens = Vec::<String>::new();
        

        return tokens;
    }
}