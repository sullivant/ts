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
    let date_re = Regex::new(r"^\d{4}-\d{2}-\d{2}.*").unwrap();

    for line in stdin.lock().lines() {
        let this_line = line.unwrap();
        if date_re.is_match(this_line.as_str()) {
            println!("{}", this_line);
        } else {
            println!(
                "{} {}",
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                this_line
            );
        }
    }

    Ok(())
}
