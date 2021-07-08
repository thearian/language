use regex::Regex;


pub fn compile(text: &String) -> String {
    let statements: Vec<[&str; 2]> = vec!(
        [r"untill (?P<t>.+)", "while not $t:"],
        [r"get (?P<t>.+)", "input($t)"],
        [r"am|is|are (?P<t>.+)", "==$t"],
        [r"do|then (?P<t>.+)", "$t"],
    );
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
