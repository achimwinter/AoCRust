fn main() {
    solve(slurp::read_all_lines("input").unwrap());
}

fn get_val(s: &str, one: char) -> i32
{
    let mut res = 0;
    let mut pow = 1;
    for i in 0..s.len() {
        if s.chars().nth(s.len() - i - 1).unwrap() == one { res |= pow; }
        pow <<= 1;
    }
    res
}

pub fn solve(data: Vec<String>) -> (i32, i32) {
    let mut res = (0, 0);
    let mut seats = vec![];

    for line in data {
        let number = get_val(&line[..7], 'B') * 8 + get_val(&line[7..], 'R');
        seats.push(number);

        if number > res.0 { res.0 = number; }
    }

    seats.sort();

    for v in *seats.first().unwrap()..*seats.last().unwrap()
    {
        if !seats.contains(&v) { res.1 = v; }
    }

    println!("part1:{}", res.0);
    println!("part2:{}", res.1);

    res
}