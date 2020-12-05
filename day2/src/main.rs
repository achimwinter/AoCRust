use std::io;
use std::fs::File;
use std::io::Read;
use nom::IResult;
use nom::combinator::map;
use nom::character::complete::digit1;



fn main() {
    let input = slurp::read_all_to_string("input").unwrap();

    let mut counter: u32 = 0;
    let mut counter_valid = 0;

    for password in input.lines() {
        let mut pattern: Vec<&str> = password.split("-").collect();
        let min = *pattern.get(0).unwrap();
        pattern = (*pattern.get(1).unwrap()).split_whitespace().collect();
        let max = *pattern.get(0).unwrap();
        pattern = (*pattern.get(1).unwrap()).split(":").collect();
        let char = (*pattern.get(0).unwrap()).chars().next().expect("");
        pattern = password.split(":").collect();
        let pass = (*pattern.get(1).unwrap()).trim();

        let number = pass.chars()
            .filter(|&c| c.eq(&char))
            .count();
        if number.ge(&(min.parse::<u32>().unwrap() as usize)) && number.le(&(max.parse::<u32>().unwrap() as usize)) {
            counter += 1
        }

        let first_correct = pass.chars().nth(min.parse::<u32>().unwrap() as usize - 1).unwrap().eq(&char);
        let second_correct = pass.chars().nth(max.parse::<u32>().unwrap() as usize - 1).unwrap().eq(&char);

        if first_correct && !second_correct { counter_valid +=1 }
        else if !first_correct && second_correct { counter_valid += 1 }

        println!("Searching char: {}, First char: {}, Second Char: {}, Result {}", char, pass.chars().nth(min.parse::<u32>().unwrap() as usize - 1).unwrap(), pass.chars().nth(max.parse::<u32>().unwrap() as usize - 1).unwrap(), first_correct && second_correct);
        // print!("{}", first_correct);
    }
    println!("Number of wrong Passwords: {}\n Number of correct Passwords: {}", counter, counter_valid);
}