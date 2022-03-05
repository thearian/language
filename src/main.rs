use std::env;
use std::fs;
use std::io::Write;
use std::process::Command;
use std::process::Output;
use regex::Regex;
use json;


fn main() {
    let (content, destination, language, should_run) = get_and_read_inputs();

    let compiled = compile(language, content);

    let result = write_file(&destination, &compiled);
    print_result(result, &destination);

    if should_run {
        let output: Output = run_file(&destination);
        println!("Successfully ran with this output: \n{}",
            buffer_to_string(&output.stdout)
        );
    }
}


pub fn compile(language: String, source: String) -> String {
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


fn get_and_read_inputs() -> (String, String, String, bool) {
    let args = get_env_args();

    if args.len() < 2 {
        panic!("\n\tExample of use:
        \n\tlanguage myfile.lang
        \n\tor
        \n\tlanguage myfile.lang myapp\n");
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

    ( content, destination, String::from("python"), should_run )
}


fn get_env_args() -> Vec<String> {
    env::args().collect()
}


fn read_file(filepath: &String) -> String {
    fs::read_to_string(filepath)
        .expect(&format!("\n\tFaild to read the file {}", filepath))
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