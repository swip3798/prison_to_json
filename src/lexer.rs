extern crate regex;

pub fn tokenize(raw_prison: String)->Vec<String> {
    let mut tokens = Vec::<String>::new();
    for mat in regex::Regex::new("(\"(.+)\"|\\S+)").unwrap().find_iter(raw_prison.as_str()) {
        let mut matched = mat.as_str().to_string();
        if matched.chars().next().unwrap() == '"' {
            matched = matched[1..matched.len()-1].to_string();
        }
        tokens.push(matched);
    }
    return tokens;
}