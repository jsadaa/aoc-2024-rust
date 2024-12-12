use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Rule {
    left: i32,
    right: i32,
}

impl Rule {
    fn new(left: i32, right: i32) -> Self {
        Self { left, right }
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
        Self { pages }
    }

    fn respect(&self, rule: &Rule) -> bool {
        self.pages.iter().position(|&p| p == rule.left()).unwrap()
            < self.pages.iter().position(|&p| p == rule.right()).unwrap()
    }

    fn mid(&self) -> Option<&i32> {
        self.pages.get(self.pages.len() / 2)
    }

    fn set_pages(mut self, pages: Vec<i32>) -> Self {
        self.pages = pages;
        self
    }
}

#[derive(Debug, Clone)]
struct Node {
    value: i32,
    outgoing: HashSet<i32>,
    incoming: HashSet<i32>,
}

impl Node {
    fn new(value: i32) -> Self {
        Self {
            value,
            outgoing: HashSet::new(),
            incoming: HashSet::new(),
        }
    }

    fn add_incoming(&mut self, from_value: i32) {
        self.incoming.insert(from_value);
    }

    fn add_outgoing(&mut self, to_value: i32) {
        self.outgoing.insert(to_value);
    }

    fn remove_incoming(&mut self, from_value: i32) {
        self.incoming.remove(&from_value);
    }

    fn remove_outgoing(&mut self, to_value: i32) {
        self.outgoing.remove(&to_value);
    }

    fn incoming_degree(&self) -> usize {
        self.incoming.len()
    }
}

pub(crate) fn day_5_1() {
    let rules: Vec<Rule> = include_str!("../data/day5.txt")
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|s| {
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
        .map(|s| Update::new(s.split(',').map(|s| s.parse::<i32>().unwrap()).collect()))
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

pub(crate) fn day_5_2() {
    use std::collections::{HashMap, HashSet, VecDeque};

    let rules: Vec<Rule> = include_str!("../data/day5.txt")
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|s| {
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
        .map(|s| Update::new(s.split(',').map(|s| s.parse::<i32>().unwrap()).collect()))
        .collect();

    let total: i32 = updates
        .into_iter()
        .filter(|update| {
            rules
                .iter()
                .filter(|rule| rule.apply_to(update))
                .any(|rule| !update.respect(rule))
        })
        .map(|update| {
            let mut nodes: HashMap<i32, Node> = update
                .pages
                .iter()
                .map(|&page| (page, Node::new(page)))
                .collect();

            let applied_rules: Vec<&Rule> =
                rules.iter().filter(|rule| rule.apply_to(&update)).collect();

            for rule in applied_rules {
                nodes
                    .get_mut(&rule.left())
                    .unwrap()
                    .add_outgoing(rule.right());
                nodes
                    .get_mut(&rule.right())
                    .unwrap()
                    .add_incoming(rule.left());
            }

            let mut reordered_pages: Vec<i32> = Vec::new();
            let total_nodes = nodes.len();

            let mut no_incoming: VecDeque<i32> = nodes
                .values()
                .filter(|node| node.incoming_degree() == 0)
                .map(|node| node.value)
                .collect();

            while let Some(n) = no_incoming.pop_front() {
                reordered_pages.push(n);

                let outgoing: Vec<i32> = nodes.get(&n).unwrap().outgoing.iter().cloned().collect();

                for m in outgoing {
                    nodes.get_mut(&m).unwrap().remove_incoming(n);
                    if nodes.get(&m).unwrap().incoming_degree() == 0 {
                        no_incoming.push_back(m);
                    }
                }
            }

            assert!(
                reordered_pages.len() == total_nodes,
                "Cycle detected in graph"
            );

            update.set_pages(reordered_pages)
        })
        .map(|update| *update.mid().unwrap())
        .sum();

    println!("{total:#?}");
}
