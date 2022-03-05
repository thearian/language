use std::env;
use std::fs;
use std::io::Write;
use regex::Regex;
use json;
use clap::Parser;

/// Programming language simplifier
#[derive(Parser, Debug)]
#[clap(
    author = "Arian Mirahmadi thearian@github mirarianmir@gmail.com",
    version = "0.0.1",
    about = "Programming language simplifier",
    long_about = None
)]
struct Args {
    /// Input file with .lang extention
    file: String,

    /// Programming language
    #[clap(short, long, default_value = "python")]
    language: String,

    /// Output file name
    #[clap(short, long, default_value = "app")]
    destination: String
}

fn main() {
    // let (content, destination, language, should_run) = get_and_read_inputs();
    let args = Args::parse();

    let content = read_file(&args.file);
    let compiled = compile(&args.language, content);

    let result = write_file(&args.destination, &args.language, &compiled);
    print_result(result, &args.destination);
}


pub fn compile(language: &String, source: String) -> String {
    let app_dir = env::current_exe()
        .unwrap();
    let root_dir = app_dir .to_str()
        .unwrap()
        .split("target")
        .collect::<Vec<&str>>()[0];
    let patterns_string = read_file(
        &format!("{}/patterns/{}.json", root_dir, language)
    );
    let mut text = source.to_owned();
    let patterns : json::JsonValue = json::parse(&patterns_string).unwrap();

    for pattern in patterns.entries() {
        let regex_pattern = Regex::new(&pattern.0).unwrap();
        let replacement = pattern.1.as_str().unwrap();

        let changed_text = regex_pattern.replace_all(&text, replacement);
        if changed_text != text {
            text = String::from(changed_text);
        }
    }

    return text;
}



fn read_file(filepath: &String) -> String {
    fs::read_to_string(filepath)
        .expect(&format!("\n\tFaild to read the file {}", filepath))
}


fn write_file(destination: &String, language: &String, content: &String) -> std::io::Result<()> {
    let extention = match language.as_str() {
        "python" => "py",
        other => other
    };
    let filepath = format!("{}.{}", destination, extention);
    let mut file = fs::File::create(filepath)?;
    file.write_all(content.as_bytes() as &[u8])?;
    Ok(())
}


fn print_result(result: std::io::Result<()>, destination: &String) {
    match result {
        Ok(_) => println!("Successfully compiled at '{}'",destination),
        Err(err) => println!("\n\tFaild Compiling\n\t{}\n", err),
    }
}