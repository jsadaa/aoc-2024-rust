use std::cmp::max;
use std::cmp::min;

#[derive(Debug, Clone)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn new(levels: Vec<i32>) -> Self {
        Report { levels }
    }

    fn is_safe(&self) -> bool {
        (self.is_increasing() || self.is_decreasing()) && self.deltas_are_safe()
    }

    fn is_dampen_safe(&self) -> bool {
        self.is_safe() || self.can_dampen()
    }

    fn deltas_are_safe(&self) -> bool {
        self.levels.windows(2).all(|l| {
            (max(l[0], l[1]) - min(l[0], l[1]) >= 1) && (max(l[0], l[1]) - min(l[0], l[1]) <= 3)
        })
    }

    fn is_increasing(&self) -> bool {
        self.levels.windows(2).all(|l| l[0] <= l[1])
    }

    fn is_decreasing(&self) -> bool {
        self.levels.windows(2).all(|l| l[0] >= l[1])
    }

    fn can_dampen(&self) -> bool {
        for (i, _) in self.levels.iter().enumerate() {
            let mut candidate = self.clone();
            candidate.levels.remove(i);

            if candidate.is_safe() {
                return true;
            }
        }

        false
    }
}

pub(crate) fn day_2_1() {
    let reports: Vec<Report> = include_str!("../data/day2.txt")
        .lines()
        .map(|line| {
            Report::new(
                line.split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect(),
            )
        })
        .collect();

    let res = reports.iter().filter(|report| report.is_safe()).count();

    println!("Number of safe reports = {res}");
}

pub(crate) fn day_2_2() {
    let reports: Vec<Report> = include_str!("../data/day2.txt")
        .lines()
        .map(|line| {
            Report::new(
                line.split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect(),
            )
        })
        .collect();

    let res = reports
        .iter()
        .filter(|report| report.is_dampen_safe())
        .count();

    println!("Number of dampen safe reports = {res}");
}
