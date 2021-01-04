use std::io;
use std::io::prelude::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;


fn remove_last_word(s: &str) -> String {
    let fields:Vec<&str> = s.split(' ').collect();
    return fields[..fields.len() - 1].join(" ");
}

fn remove_first_and_last_word(s: &str) -> String {
    let fields:Vec<&str> = s.split(' ').collect();
    return fields[1..fields.len() - 1].join(" ");
}

fn main() {
    let mut order = HashMap::<String, Vec<String>>::new();

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        let fields:Vec<&str> = line.split(" contain ").collect();

        if fields[1].starts_with("no other") {
            continue;
        }

        let color = remove_last_word(fields[0]);

        let sub_colors = fields[1].split(", ").map(|x| remove_first_and_last_word(x));
        for sub_color in sub_colors {
            order.entry(sub_color).or_default().push(color.clone());
        }
    }

    let mut queue = VecDeque::new();
    let mut good_colors = HashSet::new();
    queue.push_back("shiny gold");

    while !queue.is_empty() {
        let color = queue.pop_front().unwrap();
        let children = order.get(color);
        match children {
            None => continue,
            Some(real_children) => {
                for c in real_children {
                    queue.push_back(c);
                    good_colors.insert(c);
                }
            }
        }
    }

    println!("{}", good_colors.len());
}