/// Available operators for combining numbers in equations
enum Operator {
    Add,
    Mul,
    Concat,
}

/// Represents an equation with a target value and list of numbers
struct Equation {
    target: i64,
    numbers: Vec<i64>,
}

impl Equation {
    /// Creates a new Equation with the given target and numbers
    fn new(target: i64, numbers: Vec<i64>) -> Self {
        Self { target, numbers }
    }

    /// Generates all possible combinations of operators (+ and *) between numbers
    /// Uses bits to represent combinations:
    /// - For n numbers, we need n-1 operators between them
    /// - Each bit position represents one operator slot
    /// - 0 bit = Add operator, 1 bit = Multiply operator
    /// - Total combinations = 2^(n-1) since each position has 2 possibilities
    fn gen_combs_two_ops(&self) -> Vec<Vec<Operator>> {
        let n = u32::try_from(self.numbers.len()).unwrap();
        // Calculate total combinations needed: 2^(n-1)
        let total_combs = 2u32.pow(n - 1);
        let mut combs: Vec<Vec<Operator>> = vec![];

        // Generate each combination by counting from 0 to total_comb-1
        // Each number's binary representation is a unique operator combination
        for comb in 0..total_combs {
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

    /// Generates all possible combinations of operators (+, * and ||) between numbers
    /// Uses base-3 numbers to represent combinations:
    /// - For n numbers, we need n-1 operators between them
    /// - Each position represents one operator slot
    /// - 0 = Add, 1 = Multiply, 2 = Concatenate
    /// - Total combinations = 3^(n-1) since each position has 3 possibilities
    fn gen_combs_three_ops(&self) -> Vec<Vec<Operator>> {
        let n = u32::try_from(self.numbers.len()).unwrap();
        // Calculate total combinations needed: 3^(n-1)
        let total_combs = 3u32.pow(n - 1);
        let mut combs: Vec<Vec<Operator>> = vec![];

        // Generate each combination by counting from 0 to total_comb-1
        // Each number in base-3 represents a unique operator combination
        for comb in 0..total_combs {
            let mut operators = vec![];
            let mut temp = comb;

            // Extract operator for each position using modulo-3 operations
            for _ in 0..(n - 1) {
                // Get remainder when divided by 3 to determine operator
                // 0 = Add, 1 = Multiply, 2 = Concatenate
                let op_code = temp % 3;

                match op_code {
                    0 => operators.push(Operator::Add),
                    1 => operators.push(Operator::Mul),
                    2 => operators.push(Operator::Concat),
                    _ => panic!("invalid opcode : {op_code}"),
                }

                // Integer divide by 3 to get next digit
                temp /= 3;
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
                Operator::Concat => {
                    acc * i64::try_from(10u64.pow(Self::nb_digits(self.numbers[i + 1]))).unwrap()
                        + self.numbers[i + 1]
                }
            })
    }

    /// Determine if the equation is possible by trying all possible operator combinations to find one that matches target
    /// Returns Ok if a valid combination is found, Err otherwise
    fn eval_two_ops(&self) -> Result<(), ()> {
        for comb in &self.gen_combs_two_ops() {
            if self.eval_comb(comb) == self.target {
                return Ok(());
            }
        }

        Err(())
    }

    /// Determine if the equation is possible by trying all possible operator combinations to find one that matches target
    /// Returns Ok if a valid combination is found, Err otherwise
    fn eval_three_ops(&self) -> Result<(), ()> {
        for comb in &self.gen_combs_three_ops() {
            if self.eval_comb(comb) == self.target {
                return Ok(());
            }
        }

        Err(())
    }

    /// Calculates the number of decimal digits in the given number
    fn nb_digits(n: i64) -> u32 {
        if n == 0 {
            return 1;
        }

        n.abs().ilog10() + 1
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

    let total: i64 = equations
        .iter()
        .filter(|eq| eq.eval_two_ops().is_ok())
        .map(|eq| eq.target)
        .sum();

    println!("Total calibration result : {total}");
}

pub(crate) fn day_7_2() {
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

    let total: i64 = equations
        .iter()
        .filter(|eq| eq.eval_three_ops().is_ok())
        .map(|eq| eq.target)
        .sum();

    println!("Total calibration result : {total}");
}
