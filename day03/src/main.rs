use std::env;
use std::io;
use std::io::prelude::BufRead;

fn calc_tree_count(field: &Vec<Vec<char>>, step_x: usize, step_y: usize) -> usize {
    let height = field.len();
    let width = field[0].len();
    let mut x = 0;
    let mut y = 0;

    let mut tree_cnt = 0;

    while y < height {
        if field[y][x] == '#' {
            tree_cnt += 1;
        }

        y += step_y;
        x = (x + step_x) % width;
    }

    tree_cnt
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Len args: {}", args.len());

    let mut field:Vec<Vec<char>> = Vec::new();

    for line in io::stdin().lock().lines() {
        let l2:Vec<char> = line.unwrap().trim().chars().collect();
        field.push(l2);
    }

    if args[1] == "1" { // part 1
        let tree_cnt = calc_tree_count(&field, 3, 1);
        println!("{}", tree_cnt);

    } else { // part 2
        let tree_cnt1 = calc_tree_count(&field, 1, 1);
        let tree_cnt2 = calc_tree_count(&field, 3, 1);
        let tree_cnt3 = calc_tree_count(&field, 5, 1);
        let tree_cnt4 = calc_tree_count(&field, 7, 1);
        let tree_cnt5 = calc_tree_count(&field, 1, 2);
        println!("{} {} {} {} {}    {}", tree_cnt1, tree_cnt2,
            tree_cnt3, tree_cnt4, tree_cnt5,
            tree_cnt1 * tree_cnt2 * tree_cnt3 * tree_cnt4 * tree_cnt5);

    }
}