use std::env;
use std::io;
use std::io::prelude::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_day1 = args.len() < 2 || args[1] == "1";

    let mut max_id:usize = 0;
    let mut seen = [false; 1024];

    for line in io::stdin().lock().lines() {
        let mut id:usize = 0;
        for c in line.unwrap().trim().chars() {
            let bit = if c == 'B' || c == 'R' { 1 } else { 0 };
            id = (id << 1) | bit;
        }

        if id > max_id {
            max_id = id;
        }

        seen[id] = true;
    }

    if is_day1 {
        println!("{}", max_id);
    } else {
        for id in 1..1023 {
            if seen[id - 1] && seen[id + 1] && !seen[id] {
                println!("{}", id);
                break
            }
        }
    }
}
