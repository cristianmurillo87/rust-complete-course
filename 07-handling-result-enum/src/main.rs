use std::{fs, io::Error};

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        // There's not need for an specific value here
        // returning () as the value for Ok is enough in this case
        return Ok(());
    }
    Err(Error::other("Invalid email!"))
}

fn read_file(path: &str) -> Result<String, Error> {
    fs::read_to_string(path)
}

fn extract_log_errors(errors: &str) -> Vec<&str> {
    errors
            .split('\n')
            .into_iter()
            .filter(|line| line.starts_with("ERROR"))
            .collect()
}

fn main() {
    // path to file relative to the project root
    let text = fs::read_to_string("logs.txt");

    match text {
        Ok(result) =>  println!("{:#?}", result),
        Err(explanation) => println!("An error occurred while trying to read the file!!\n{:#?}", explanation)
    }

    match validate_email(String::from("something@provider.com")) {
        // _ indicates the value inside the result is not relevant for the evaluation
        Ok(_) => println!("Email is valid"),
        Err(_) => println!("Invalid email!")
    }

    match read_file("logs.txt") {
        Ok(result) => {
            let error_lines = extract_log_errors(&result);
            if error_lines.len() == 0 {
                println!("No errors found in log file");
            } else {
                for line in error_lines {
                    println!("{}", line);
                }
            }
        },
        Err(explanation) => println!("An error occurred while trying to read the file!!\n{:#?}", explanation)
    }
}
