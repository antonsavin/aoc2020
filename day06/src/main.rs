use std::env;
use std::io;
use std::iter::FromIterator;
use std::io::prelude::BufRead;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_day1 = args.len() < 2 || args[1] == "1";

    let mut any_yes_questions = HashSet::new();
    let mut all_yes_questions = HashSet::new();
    let mut count_sum = 0;
    let mut was_empty_line = true;

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.len() == 0 {
            if !was_empty_line {
                if is_day1 {
                    count_sum += any_yes_questions.len();
                } else {
                    count_sum += all_yes_questions.len();
                }
            }

            was_empty_line = true;
            any_yes_questions.clear();
            all_yes_questions.clear();
            continue;
        }

        let this_yes_questions = HashSet::<char>::from_iter(line.chars());
        for c in &this_yes_questions {
            any_yes_questions.insert(*c);
        }

        if was_empty_line {
            all_yes_questions = this_yes_questions;
        } else {
            all_yes_questions = all_yes_questions.intersection(&this_yes_questions).cloned().collect();
        }

        was_empty_line = false;
    }

    if is_day1 {
        count_sum += any_yes_questions.len();
    } else if !was_empty_line {
        count_sum += all_yes_questions.len();
    }

    println!("{}", count_sum);

}