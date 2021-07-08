use regex::Regex;

mod loops;
mod functions;


pub fn compile(text: &String) -> String {
    let mut statements: Vec<[&str; 2]> = Vec::new();

    statements.extend(&loops::statements());
    statements.extend(&functions::statements());

    map_statements(text, &statements)
}

fn map_statements(text: &String,statements: &Vec<[&str; 2]>) -> String {
    let mut result = String::from(text);
    for statement in statements.into_iter() {
        result = replace_statement(&result, statement[0], statement[1])
    }
    result
}

fn replace_statement(source: &String, pattern: &str, replacement: &str) -> String {
    let statement = Regex::new(pattern).unwrap();
    let replaced = statement.replace_all(source, replacement);
    String::from(replaced)
}
