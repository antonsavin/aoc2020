use std::io;
use std::io::prelude::BufRead;
use std::collections::HashSet;

fn main() {
    let mut nums = HashSet::<i32>::new();

    for line in io::stdin().lock().lines() {
        let num:i32 = line.unwrap().trim().parse().expect("Failed to parse");
        if nums.contains(&(2020 - num)) {
            println!("{} {} {}", (2020 - num), num, (2020 - num) * num);
            return
        }

        nums.insert(num);
    }
}
