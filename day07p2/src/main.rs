use std::io;
use std::io::prelude::BufRead;
use std::collections::HashMap;

fn remove_last_word(s: &str) -> String {
    let fields:Vec<&str> = s.split(' ').collect();
    return fields[..fields.len() - 1].join(" ");
}

fn subtree_size(tree:&HashMap::<String, Vec<(usize, String)>>, color:&str) -> usize {
    let children = tree.get(color);
    match children {
        None => 0,
        Some(real_children) => {
            let mut res = 0;
            for (cnt, c) in real_children {
                res += subtree_size(tree, c) * cnt + cnt;
            }
            res
        }
    }
}

fn main() {
    let mut order = HashMap::<String, Vec<(usize, String)>>::new();

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        let fields:Vec<&str> = line.split(" contain ").collect();

        if fields[1].starts_with("no other") {
            continue;
        }

        let color = remove_last_word(fields[0]);
        let children = order.entry(color.clone()).or_default();

        let sub_colors:Vec<Vec<String>> = fields[1]
            .split(", ")
            .map(|x| remove_last_word(x))
            .map(|x| x.splitn(2, " ").map(|x| String::from(x)).collect())
            .collect();
        for sub_color in sub_colors {
            let cnt:usize = sub_color[0].parse().unwrap();
            children.push((cnt, String::from(&sub_color[1])));
        }
    }

    println!("{}", subtree_size(&order, "shiny gold"));
}