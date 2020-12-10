use std::collections::HashMap;
use std::ops::Deref;
use regex::Regex;

fn main() {
    let first = get_count("input", false);
    let second = get_count("input", true);

    println!("First : {}\n Second: {}", first, second);
}


fn is_valid(lines: &str, check_values: bool) -> bool {
    let mut values: HashMap<&str, &str> = HashMap::new();
    let valid_keys = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];

    for pair in lines.split(|c| c == ' ' || c == '\n') {
        let mut pit = pair.split( ':');
        let field = pit.next().unwrap();
        let value = pit.next().unwrap();
        values.insert(field, value);
    }

    for key in valid_keys.iter() {
        if values.contains_key(key.deref()) {
            if check_values {
                let value = values.get(key.deref()).unwrap();
                if match key.deref() {
                    "ecl" => check_ecl(value),
                    "pid" => check_pid(value),
                    "eyr" => check_eyr(value),
                    "hcl" => check_hcl(value),
                    "byr" => check_byr(value),
                    "iyr" => check_iyr(value),
                    "hgt" => check_hgt(value),
                    "cid" => continue,
                    &_ => panic!("Unknown Key")
                } {
                } else {
                    return false
                }
            }
        } else {
            return false;
        }
    }
    true
}

fn check_hgt(value: &str) -> bool {
    let regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    if !regex.is_match(value) {
        return false
    } else {
        let caps = regex.captures(value).unwrap();
        let n: i32 = (&caps[1]).parse().unwrap();
        let unit: &str = &caps[2];
        (unit == "cm" && n >= 150 && n <= 193) || (unit == "in" && n >= 59 && n <= 76)
    }
}

fn check_iyr(value: &str) -> bool {
    (2010..=2020).contains(&value.parse().unwrap())
}

fn check_byr(value: &str) -> bool {
    (1920..=2002).contains(&value.parse().unwrap())
}

fn check_hcl(value: &str) -> bool {
    Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(value)
}

fn check_eyr(value: &str) -> bool {
    (2020..=2030).contains(&value.parse().unwrap())
}

fn check_ecl(value: &str) -> bool {
    let colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    colors.contains(&value)
}

fn check_pid(value: &str) -> bool {
    Regex::new(r"^\d\d\d\d\d\d\d\d\d$").unwrap().is_match(value)
}

// fn parse_string(mut values: &mut HashMap<&str, &str>, field: &str) -> String {
//     values.get(field).is_some().to_string()
// }

// fn parse_num(mut values: &mut HashMap<&str, &str>, field: &str) -> Option<u32> {
//     const RADIX: u32 = 10;
//     values.get(field)
//         .map(|str| str.chars()
//             .filter(|c| c.is_numeric())
//             .map(|c| c.to_digit(RADIX).unwrap())
//             .sum::<u32>()
//         )
// }

fn get_count(filename: &str, check_values: bool) -> u32 {
    slurp::read_all_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|lines| is_valid(lines, check_values))
        .filter(|p| p == &true)
        .count() as u32
}
