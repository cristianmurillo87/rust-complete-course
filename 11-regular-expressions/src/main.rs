use clap::Parser;
use regex::Regex;

#[derive(Debug, Parser)]
// properties can also be given in separate #[command(...)] lines
// e.g
// #[command(name="Some name")]
// #[command(version="0.1")]
#[command(name = "grep-lite", version = "0.1", about="Returns the line that matches the given expression", long_about=None)]
struct AppArgs {
    // short -p
    // long --pattern
    #[arg(short, long, value_name = "pattern", required = true)]
    pattern: String,
}

// after running cargo build
// test the application by running the following example
// target/debug/rust-regular-expressions --pattern="pictures"
fn main() {
    let cli = AppArgs::parse();

    let pattern = cli.pattern.as_ref();
    let reg_exp = Regex::new(pattern).unwrap();
    let mut found = false;

    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        let contains_substring = reg_exp.find(line);

        match contains_substring {
            Some(result) => {
                found = true;
                println!(
                    "{}\n{}-{} -> {}",
                    line,
                    result.start(),
                    result.end(),
                    result.as_str()
                )
            }
            None => (),
        }
    }

    if !found {
        println!("Pattern: {} not found", pattern);
    }
}
