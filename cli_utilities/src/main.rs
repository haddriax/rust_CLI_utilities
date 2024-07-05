use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::Path;

fn get_user_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_owned())
}

fn print_file_content_by_line<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => println!("{}", line),
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    Ok(())
}

fn user_loop() {
    loop {
        println!("{}", "Path of the file to echo:");

        let input = get_user_input().expect("Failed to read from stdin.");

        if input == "exit" {
            break;
        }

        if input.is_empty() {
            continue;
        }

        if !Path::new(&input).exists() {
            eprintln!("File does not exist: {}", input);
            continue;
        }

        if !Path::new(&input).is_file() {
            eprintln!("Path is not a file: {}", input);
            continue;
        }

        println!("{}", &input);

        print_file_content_by_line(&input).unwrap_or_else(|err| {
            eprintln!("Error while opening the file '{}': {}", input, err);
            std::process::exit(1);
        });
    }
}

fn main() {
    user_loop();
}




