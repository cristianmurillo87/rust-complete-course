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
    // the try operator is used (?) can only be used after functions that return a 'Result' enum,
    // if the result is Ok(), it returns the value contained within it,
    // otherwise it returns the Error variant
    let result = fs::read_to_string(path)?;
    Ok(result)
}

fn write_file(path: &str, content: String) -> Result<(), Error> {
    fs::write(path, content)
}

fn extract_log_errors(errors: &str) -> Vec<&str> {
    errors
            .split('\n')
            .into_iter()
            .filter(|line| line.starts_with("ERROR"))
            .collect()
}

fn main(){
    match validate_email(String::from("something@provider.com")) {
        // _ indicates the value inside the result is not relevant for the evaluation
        Ok(_) => println!("Email is valid"),
        Err(_) => println!("Invalid email!")
    }

     // path to file relative to the project root
    match read_file("logs.txt") {
        Ok(result) => {
            let error_lines = extract_log_errors(&result);
            if error_lines.len() == 0 {
                println!("No errors found in log file");
            } else {
                write_file("errors.txt", error_lines.join("\n")).expect("Saving errors to errors.txt failed");
            }
        },
        Err(explanation) => println!("An error occurred while trying to read the file!!\n{:#?}", explanation)
    }
}
