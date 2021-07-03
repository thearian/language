use std::env;
use std::fs;
use std::io::Write;
use std::process::Command;
use std::process::Output;


fn main() {
    let (content, destination, should_run) = get_and_read_inputs();

    let compiled = compile_content(&content);

    let result = write_file(&destination, &compiled);
    print_result(result, &destination);

    if should_run {
        let output: Output = run_file(&destination);
        println!("Successfully ran with this output: \n{}",
            buffer_to_string(&output.stdout)
        );
    }
}


fn get_and_read_inputs() -> (String, String, bool) {
    let args = get_env_args();

    if args.len() < 2 {
        panic!("\n\tExample of use:
        \n\tlanguage myfile
        \n\tor
        \n\tlanguage myfile myapp\n");
    }

    let filepath = &args[1];
    let content = read_file(filepath);

    println!("Successfully read from '{}'",filepath);

    let mut destination = String::from("app");
    if args.len() > 2 {
        destination = args[2].to_owned();
    }

    let should_run: bool = args.iter().any(
        |arg| arg=="-r" || arg=="--run"
    );

    ( content, destination, should_run )
}


fn get_env_args() -> Vec<String> {
    env::args().collect()
}


fn read_file(filepath: &String) -> String {
    fs::read_to_string(filepath.to_owned()+".lang")
        .expect("\n\tFaild to read the file")
}


fn write_file(destination: &String, content: &String) -> std::io::Result<()> {
    let mut file = fs::File::create(destination.to_owned() + ".py")?;
    file.write_all(content.as_bytes() as &[u8])?;
    Ok(())
}


fn print_result(result: std::io::Result<()>, destination: &String) {
    match result {
        Ok(_) => println!("Successfully compiled at '{}'",destination),
        Err(err) => println!("\n\tFaild Compiling\n\t{}\n", err),
    }
}

fn run_file(destination: &String) -> Output {
    let mut command: String = "py ".to_owned();
    command.push_str(&destination);
    command.push_str(".py");
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", &command])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output()
                .expect("failed to execute process")
    };
    output
}

fn buffer_to_string(buffer: &[u8]) -> &str {
    return
        match std::str::from_utf8(buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
}

fn compile_content(content: &String) -> String {
    let lines: String = iter_lines(&content);
    iter_words(&lines)
}

fn iter_lines(text: &String) -> String {
    let lines: std::str::SplitWhitespace = text.split_whitespace();
    lines.map(|line| {
        let mut owned_line = line.to_owned();
        owned_line.push_str("\n");
        owned_line
    }).collect()
}

fn iter_words(text: &String) -> String {
    let words: std::str::SplitWhitespace = text.split_whitespace();
    words.map(|word| {
        let mut owned_word = word.to_owned();
        owned_word = map_word(&owned_word);
        owned_word.push_str(" ");
        owned_word
    }).collect()
}

fn map_word(word: &String) -> String {
    let maped = match word.as_str() {
        "untill" => "while not",
        default => default,
    };
    String::from(maped)
}