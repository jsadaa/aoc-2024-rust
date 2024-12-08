use std::{
    cmp::{max, min},
    str::Lines,
};

pub(crate) fn day_1_1() {
    let lines: Lines = include_str!("../data/day1.txt").lines();

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    lines.for_each(|line| {
        left.push(line.split_at(5).0.parse::<i32>().unwrap());
        right.push(line.split_at(5).1.trim().parse::<i32>().unwrap());
    });

    left.sort_unstable();
    right.sort_unstable();

    let mut total_distance = 0;

    for i in 0..left.len() {
        let distance = max(left[i], right[i]) - min(left[i], right[i]);
        total_distance += distance;
    }

    println!("Total distance = {total_distance}");
}

pub(crate) fn day_1_2() {
    let lines: Lines = include_str!("../data/day1.txt").lines();

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    lines.for_each(|line| {
        left.push(line.split_at(5).0.parse::<i32>().unwrap());
        right.push(line.split_at(5).1.trim().parse::<i32>().unwrap());
    });

    let similarity = left.iter().fold(0, |acc, i| {
        acc + (i32::try_from(right.iter().filter(|j| i == *j).count()).unwrap() * i)
    });

    println!("Similarity = {similarity}");
}
