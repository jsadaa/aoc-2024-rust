#[derive(Debug)]
struct Rule {
    left: i32,
    right: i32,
}

impl Rule {
    fn new(left: i32, right: i32) -> Self {
        Rule { left, right }
    }

    fn left(&self) -> i32 {
        self.left
    }

    fn right(&self) -> i32 {
        self.right
    }

    fn apply_to(&self, update: &Update) -> bool {
        update.pages.contains(&self.left) && update.pages.contains(&self.right)
    }
}

#[derive(Debug)]
struct Update {
    pages: Vec<i32>,
}

impl Update {
    fn new(pages: Vec<i32>) -> Self {
        Update { pages }
    }

    fn respect(&self, rule: &Rule) -> bool {
        self.pages.iter().position(|&p| p == rule.left()).unwrap()
            < self.pages.iter().position(|&p| p == rule.right()).unwrap()
    }

    fn mid(&self) -> Option<&i32> {
        self.pages.get(self.pages.len() / 2)
    }
}

pub(crate) fn day_5_1() {
    let rules: Vec<Rule> = include_str!("../data/day5.txt")
        .lines()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|s: &str| {
            let split: Vec<&str> = s.split('|').collect();
            Rule::new(
                split.first().unwrap().parse::<i32>().unwrap(),
                split.last().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();

    let updates: Vec<Update> = include_str!("../data/day5.txt")
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|s: &str| Update::new(s.split(',').map(|s| s.parse::<i32>().unwrap()).collect()))
        .collect();

    let total: i32 = updates
        .iter()
        .filter(|update| {
            rules
                .iter()
                .filter(|rule| rule.apply_to(update))
                .all(|rule| update.respect(rule))
        })
        .map(|update| *update.mid().unwrap())
        .sum();

    println!("Total : {total:#?}");
}

pub(crate) fn day_5_2() {}
