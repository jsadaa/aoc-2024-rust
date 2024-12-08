use regex::Regex;
use std::str::Lines;

pub(crate) fn day_3_1() {
    let line = include_str!("../data/day3.txt")
        .lines()
        .collect::<Vec<&str>>()
        .join("\n");

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut mul = vec![];

    for (_, [op1, op2]) in re.captures_iter(&line).map(|c| c.extract()) {
        mul.push((op1.parse::<i32>().unwrap(), op2.parse::<i32>().unwrap()));
    }

    let res = mul.iter().fold(0, |acc, (op1, op2)| acc + (op1 * op2));

    println!("Total = {res}");
}

pub(crate) fn day_3_2() {
    let line = include_str!("../data/day3.txt")
        .lines()
        .collect::<Vec<&str>>()
        .join("\n");

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut capture = true;
    let mut mul = vec![];

    for cap in re.captures_iter(&line) {
        let Some(m) = cap.get(0) else {
            panic!("no match found");
        };

        let m = m.as_str();

        match m {
            "do()" => capture = true,
            "don't()" => capture = false,
            _ if capture => {
                if let (Some(op1), Some(op2)) = (cap.get(1), cap.get(2)) {
                    mul.push((
                        op1.as_str().parse::<i32>().unwrap(),
                        op2.as_str().parse::<i32>().unwrap(),
                    ));
                }
            }
            _ => (),
        }
    }

    let res = mul.iter().fold(0, |acc, (op1, op2)| acc + (op1 * op2));

    println!("Total = {res}");
}
