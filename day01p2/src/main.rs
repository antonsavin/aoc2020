use std::io;
use std::io::prelude::BufRead;
use std::collections::HashSet;

fn sum_2(need_sum:i32, nums:&Vec<i32>) ->bool {
    let mut seen_nums = HashSet::new();

    for num in nums.iter() {
        let num1 = need_sum - num;
        if seen_nums.contains(&num1) {
            let num2 = 2020 - need_sum;
            println!("{} {} {} {}", num1, num, num2, num * num1 * num2);
            return true;
        }

        seen_nums.insert(num);
    }

    false
}

fn main() {
    let mut nums = Vec::new();

    for line in io::stdin().lock().lines() {
        let num:i32 = line.unwrap().trim().parse().expect("Failed to parse");
        nums.push(num);
    }

    for x in nums.iter() {
        if sum_2(2020 - x, &nums) {
            return
        }
    }
}
