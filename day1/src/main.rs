use std::str::FromStr;

fn main() {
    // Loads an entire file of numbers as a Vector of Result<u32> structs
    let numbers = read_all::<u32>("src/numbers.txt");

    let ints = numbers
        .iter()
        .map(|x| x.as_ref().unwrap());

    let answer = ints
        .flat_map(|y| ints.cloned().map(move |x| (x, y)))
        .filter(|(x, y)| x + *y == 2020)
        .map(|(x, y)| x * y)
        .next();

    println!("{}", answer.unwrap());
    //
    // let result = numbers.iter()
    //     .map(|x| *x.as_ref().expect("Read failed"))
    //     .fold(0, |acc, x| {
    //         if acc + x == 2020 {
    //             acc * x
    //         } else { x }
    //     });

        // let my_file: Vec<String> = slurp::read_all_lines("report.txt").unwrap();
        // let ints = my_file
        //     .iter()
        //     .map(|x| x.parse::<i32>().unwrap());

        // let ints = numbers
        //     .iter()
        //     .map(|x| x.as_ref().unwrap());
        //
        // let answer = ints
        //     .flat_map(|y| ints.clone().map(move |x| (x, y)))
        //     .filter(|(x,y)| x + y == 2020)
        //     .map(|(x,y)| x*y)
        //     .next();
        println!("answer: {}", "test");
}



    // println!("{}", result);
    // for number_a in &numbers {
    //     for number_b in &numbers {
    //         for number_c in &numbers {
    //             let test = *number_a.as_ref().expect("");
    //             let test2 = *number_b.as_ref().expect("");
    //             let test3 = *number_c.as_ref().expect("");
    //             let result = test + test2 + test3;
    //             if 2020 == result {
    //                 first = test;
    //                 second = test2;
    //                 third = test3;
    //             }
    //         }
    //     }
    // }

    // println!("{} plus {} plus {} is 2020", first, second, third);
    // println!("{} multiplied by {} and {} is: {}", first, second, third, first * second * third);


fn read_all<T: FromStr>(file_name: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .collect()
}
