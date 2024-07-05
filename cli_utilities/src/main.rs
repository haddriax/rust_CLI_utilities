use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::Path;

/// Reads a line of user input from the standard input.
///
/// # Returns
///
/// `Ok(String)` - The line of user input, without trailing newline characters.
/// `Err(io::Error)` - If there is an error reading from the standard input.
///
/// # Examples
///
/// ```
/// use std::io;
///
/// fn main() {
///     let result = get_user_input();
///     match result {
///         Ok(input) => println!("User input: {}", input),
///         Err(error) => eprintln!("Error: {}", error),
///     }
/// }
/// ```
fn get_user_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_owned())
}

/// Prints the content of a file line by line.
///
/// # Arguments
///
/// * `path` - A reference to a `Path` object that represents the path of the file.
///
/// # Returns
///
/// Returns an `io::Result` indicating whether the operation was successful or not.
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

/// Continuously prompts the user for a file path and echoes the content of the file by line.
///
/// The user can exit the loop by entering "exit". The function keeps looping until the user exits.
///
/// # Panics
///
/// This function panics if it fails to read from standard input, or if there is an error while opening the file.
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




