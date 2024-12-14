enum Operator {
    Add,
    Mul,
}

struct Equation {
    target: i64,
    numbers: Vec<i64>,
}

impl Equation {
    fn new(target: i64, numbers: Vec<i64>) -> Self {
        Self { target, numbers }
    }

    /// Generates all possible combinations of operators (+ and *) between numbers
    /// Uses bits to represent combinations:
    /// - For n numbers, we need n-1 operators between them
    /// - Each bit position represents one operator slot
    /// - 0 bit = Add operator, 1 bit = Multiply operator
    /// - Total combinations = 2^(n-1) since each position has 2 possibilities
    fn gen_combs(&self) -> Vec<Vec<Operator>> {
        let n = u32::try_from(self.numbers.len()).unwrap();
        // Calculate total combinations needed: 2^(n-1)
        let n_combs = 2u32.pow(n - 1);
        let mut combs: Vec<Vec<Operator>> = vec![];

        // Generate each combination by counting from 0 to total_comb-1
        // Each number's binary representation is a unique operator combination
        for comb in 0..n_combs {
            let mut operators = vec![];

            // Extract operator for each position using bit operations
            for i in 0..(n - 1) {
                // Shift right by i to get the bit for position i
                // AND with 1 to isolate that bit
                // 0 = Add, 1 = Multiply
                if (comb >> i) & 1 == 0 {
                    operators.push(Operator::Add);
                } else {
                    operators.push(Operator::Mul);
                }
            }

            combs.push(operators);
        }

        combs
    }

    /// Evaluates a single combination of operators with the equation's numbers
    /// Processes left-to-right, no operator precedence
    fn eval_comb(&self, operators: &[Operator]) -> i64 {
        let init = self.numbers.first().unwrap().to_owned();

        operators
            .iter()
            .enumerate()
            .fold(init, |acc, (i, operator)| match operator {
                Operator::Add => acc + self.numbers[i + 1],
                Operator::Mul => acc * self.numbers[i + 1],
            })
    }

    /// Determine if the equation is possible by trying all possible operator combinations to find one that matches target
    /// Returns Ok if a valid combination is found, Err otherwise
    fn eval(&self) -> Result<(), ()> {
        for comb in &self.gen_combs() {
            if self.eval_comb(comb) == self.target {
                return Ok(());
            }
        }

        Err(())
    }
}

pub(crate) fn day_7_1() {
    let equations: Vec<Equation> = include_str!("../data/day7.txt")
        .lines()
        .map(|s| s.split(':').collect::<Vec<&str>>())
        .map(|v| {
            let target: i64 = v[0].trim().parse::<i64>().unwrap();
            let numbers: Vec<i64> = v[1]
                .split_whitespace()
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect();

            Equation::new(target, numbers)
        })
        .collect();

    let res: i64 = equations
        .iter()
        .filter(|eq| eq.eval().is_ok())
        .map(|eq| eq.target)
        .sum();

    println!("Total calibration result : {res}");
}

pub(crate) fn day_7_2() {}
