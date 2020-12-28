use std::env;
use std::io;
use std::io::prelude::BufRead;
use std::collections::HashMap;

fn is_valid_byr(s: &str) -> bool {
    // println!("Checking byr {}", s);
    let year = s.parse::<i32>();
    match year {
        Err(_) => false,
        Ok(y) => y >= 1920 && y <= 2002,
    }
}

fn is_valid_iyr(s: &str) -> bool {
    let year = s.parse::<i32>();
    match year {
        Err(_) => false,
        Ok(y) => y >= 2010 && y <= 2020,
    }
}

fn is_valid_eyr(s: &str) -> bool {
    let year = s.parse::<i32>();
    match year {
        Err(_) => false,
        Ok(y) => y >= 2020 && y <= 2030,
    }
}

fn is_valid_hgt(s: &str) -> bool {
    if s.len() < 3 {
        return false;
    }

    let suffix = &s[s.len()-2 ..];
    let prefix = &s[..s.len()-2];
    let num = prefix.parse::<i32>();
    match num {
        Err(_) => false,
        Ok(n) => {
            if suffix == "cm" {
                n >= 150 && n <= 193
            } else if suffix == "in" {
                n >= 59 && n <= 76
            } else {
                false
            }
        }
    }
}

fn is_valid_hcl(s: &str) -> bool {
    if s.len() != 7 { return false }
    let mut chars = s.chars();
    let c = chars.next();
    if c.unwrap() != '#' { return false }

    for c in chars {
        if !(c >= '0' && c <= '9' || c >= 'a' && c <= 'f') {
            return false
        }
    }

    true
}

#[test]
fn test_is_valid_hcl() {
    assert_eq!(is_valid_hcl("#0189af"), true);
    assert_eq!(is_valid_hcl("#0189ag"), false);
    assert_eq!(is_valid_hcl("00189af"), false);
    assert_eq!(is_valid_hcl("0189af"), false);
}

fn is_valid_ecl(s: &str) -> bool {
    s == "amb" || s == "blu" || s == "brn" || s == "gry"
        || s == "grn" || s == "hzl" || s == "oth"
}

fn is_valid_pid(s: &str) -> bool {
    if s.len() != 9 { return false }
    let chars = s.chars();
    for c in chars {
        if !(c >= '0' && c <= '9') {
            return false
        }
    }

    true
}

fn is_valid_passport(passport_fields: &HashMap<String, String>, required_fields:&Vec<&str>) -> bool {
    for f in required_fields {
        if !passport_fields.contains_key(*f) {
            return false
        }
    }

    true
}

fn is_valid_passport_day2(passport_fields: &HashMap<String, String>, required_fields:&Vec<&str>) -> bool {
    for f in required_fields {
        let value = passport_fields.get(*f);
        match value {
            None => return false,
            Some(s) => {
                if *f == "byr" {
                    if !is_valid_byr(s) { return false }
                } else if *f == "iyr" {
                    if !is_valid_iyr(s) { return false }
                } else if *f == "eyr" {
                    if !is_valid_eyr(s) { return false }
                } else if *f == "hgt" {
                    if !is_valid_hgt(s) { return false }
                } else if *f == "hcl" {
                    if !is_valid_hcl(s) { return false }
                } else if *f == "ecl" {
                    if !is_valid_ecl(s) { return false }
                } else if *f == "pid" {
                    if !is_valid_pid(s) { return false }
                }
            }
        }
    }

    true
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let validator:fn(passport_fields: &HashMap<String, String>, required_fields:&Vec<&str>) -> bool;
    if args[1] == "1" { // part 1
        validator = is_valid_passport;
    } else { // part 2
        validator = is_valid_passport_day2;
    }

    let mut passport_fields = HashMap::new();
    let mut valid_cnt = 0;

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.len() == 0 {
            if validator(&passport_fields, &required_fields) {
                valid_cnt += 1;
            }

            // println!("Clearing");
            passport_fields.clear();
            continue;
        }

        let tokens = line.split(" ");
        for tok in tokens {
            let kv:Vec<&str> = tok.split(':').collect();
            // println!("Adding {}", kv[0]);
            passport_fields.insert(String::from(kv[0]), String::from(kv[1]));
        }
    }

    if validator(&passport_fields, &required_fields) {
        valid_cnt += 1;
    }

    println!("{}", valid_cnt);

}