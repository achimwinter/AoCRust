use anyhow::{Result, Error};
use nom::{Parser, IResult, Map};
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::bytes::complete::tag;
use std::iter::FromIterator;

fn main() {
    let passports = Passports::read_from_file("input");

    println!("{:?}", passports);
}

#[derive(Debug)]
struct Passport {
    birth_year: u16,
    issue_year: u16,
    expriration_year: u16,
    height: u16,
    hair_color: String,
    eye_color: String,
    passport_id: u32,
    country_id: Option<u16>,
}

impl Passport {
    fn create_from_string(_lines: &str) -> Result<Self> {
        let input = lines.join(",");
        let (input, birth_year) = parse_num(input);
        // let (input, _) = tag(":")(input)?;
        // let (input, issue_year) = parse_num(input);
        // let (input, _) =
        Ok(Passport {
            birth_year: 0,
            issue_year: 0,
            expriration_year: 0,
            height: 0,
            hair_color: "".to_string(),
            eye_color: "".to_string(),
            passport_id: 0,
            country_id: None
        })
    }
}

#[derive(Debug)]
struct Passports {
    passports: Vec<Passport>,
}

impl FromIterator<Passport> for Passports {
    fn from_iter<I: IntoIterator<Item=Passport>>(iter: I) -> Self {
        let mut passports:Vec<Passport> = Vec::new();

        for passport in iter {
            passports.push(passport);
        }

        Self {
            passports,
        }
    }
}


impl Passports {
    fn read_from_file(filename: &str) -> Result<Self>{
        let passports: Vec<Passport> = slurp::read_all_to_string(filename)
            .unwrap()
            .split("\n\n")
            .map(|pass_lines| Passport::create_from_string(pass_lines).unwrap())
            .collect();

        Ok(Self { passports })
    }
}


fn parse_num(input: &str) -> IResult<&str, usize> {
    map(digit1, |digit_str: &str| {
        digit_str.parse::<usize>().unwrap()
    })(input)
}

