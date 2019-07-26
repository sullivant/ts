// ts - a fast timestamp utility
//
// Just adds a timestamp to the beginning of piped stdin if one is
// not found via a simple match.
//
// Author: Thomas Sullivan (sullivan.t@gmail.com)
//
extern crate chrono;

use chrono::prelude::*;
use regex::Regex;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    // TODO: Configure this via control file and in a safe way
    let date_re = Regex::new(r"^\d{4}-\d{2}-\d{2}.*").unwrap();

    // For each line from stdin, lets do stuff
    for line in stdin.lock().lines() {
        let this_line = line.unwrap();

        // If the line begins with a timestamp, just print it back to the output
        if date_re.is_match(this_line.as_str()) {
            println!("{}", this_line);
        } else {
            // otherwise, we can add our timestamp.
            println!(
                "{} {}",
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                this_line
            );
        }
    }

    Ok(())
}
