use std::io;
use std::io::prelude::BufRead;

fn main() {
    let mut valid_cnt = 0;

    for line in io::stdin().lock().lines() {
        let l2 = line.unwrap(); //.trim();
        let dash_pos = l2.find('-').unwrap();
        let lowest:i32 = l2[..dash_pos].parse().unwrap();
        let after_dash = &l2[dash_pos+1..];
        let space_pos = after_dash.find(' ').unwrap();
        let highest:i32 = after_dash[..space_pos].parse().unwrap();
        let the_char = after_dash.chars().nth(space_pos + 1).unwrap();
        let password = &after_dash[space_pos + 4..];
        let mut cnt = 0;
        for c in password.chars() {
            if c == the_char {
                cnt += 1;
            }
        }

        if cnt >= lowest && cnt <= highest {
            valid_cnt += 1;
        }
    }

    println!("{}", valid_cnt);
}
