pub fn map_word(word: &String) -> String {
    let maped = match word.as_str() {
        "untill:" => "while not",
        "do:" => "",
        "am" => "==",
        "are" => "==",
        default => default,
    };
    String::from(maped)
}
