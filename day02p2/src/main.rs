use std::io;
use std::io::prelude::BufRead;

fn main() {
    let mut valid_cnt = 0;

    for line in io::stdin().lock().lines() {
        let l2 = line.unwrap(); //.trim();
        let dash_pos = l2.find('-').unwrap();
        let pos1:usize = l2[..dash_pos].parse().unwrap();
        let after_dash = &l2[dash_pos+1..];
        let space_pos = after_dash.find(' ').unwrap();
        let pos2:usize = after_dash[..space_pos].parse().unwrap();
        let the_char = after_dash.chars().nth(space_pos + 1).unwrap();
        let password = &after_dash[space_pos + 4..];

        let mut cnt = 0;
        let chars:Vec<char> = password.chars().collect();
     //   println!("{} {} '{}'", pos1, pos2, password);

        if pos1 > 0 && pos1 <= chars.len() && chars[pos1 - 1] == the_char {
            cnt += 1;
        }

        if pos2 > 0 && pos2 <= chars.len() && chars[pos2 - 1] == the_char {
            cnt += 1;
        }

        if cnt == 1 {
            valid_cnt += 1;
        }
    }

    println!("{}", valid_cnt);
}
