mod regex;

use regex::Pattern;
use std::{io, io::prelude::*};

fn main() {
    const INPUT_FAILURE_MSG: &str = "failed to read input line from stdio";
    println!("Enter regex pattern:");

    let pattern = {
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect(INPUT_FAILURE_MSG);
        inp.trim().to_owned()
    };

    let pattern = Pattern::new(&pattern);

    println!("Enter input lines, we will output patterns in each line. Input \"exit\" to");
    for l in io::stdin()
        .lock()
        .lines()
        .map(|l| l.expect(INPUT_FAILURE_MSG))
    {
        if l == "exit" {
            break;
        }
        println!("matches in {}: {:?}", l.trim(), pattern.find_matches(l.trim()));
    }
}
