use std::env;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
struct StrError<'a>(&'a str);

// Error doesn't require you to implement any methods, but
// your type must also implement Debug and Display.
impl<'a> Error for StrError<'a> {}

impl<'a> fmt::Display for StrError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to the Display impl for `&str`:
        self.0.fmt(f)
    }
}

use advlib::*;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err("Usage: ./advent <day> <input>".into());
    }

    let day = args.get(1).unwrap().as_str();
    let input = args.get(2).unwrap().as_str();

    match day {
        "day1" => adv1(input),
        "day2" => adv2(input),
        _ => {
            Ok(())
        }
    }
}
