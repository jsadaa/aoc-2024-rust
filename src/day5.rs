use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct DirectedGraph {
    nodes: HashMap<i32, Node>,
}

impl DirectedGraph {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, value: i32) {
        self.nodes.insert(value, Node::new(value));
    }

    fn add_edge(&mut self, from: i32, to: i32) {
        if let Some(from_node) = self.nodes.get_mut(&from) {
            from_node.add_outgoing(to);
        }
        if let Some(to_node) = self.nodes.get_mut(&to) {
            to_node.add_incoming(from);
        }
    }

    fn topological_sort(&mut self) -> Result<Vec<i32>, &'static str> {
        let mut result = Vec::new();
        let total_nodes = self.nodes.len();

        let mut no_incoming: VecDeque<i32> = self
            .nodes
            .values()
            .filter(|node| node.incoming_degree() == 0)
            .map(|node| node.value)
            .collect();

        while let Some(n) = no_incoming.pop_front() {
            result.push(n);

            let outgoing: Vec<i32> = self
                .nodes
                .get(&n)
                .map(|node| node.outgoing.iter().copied().collect())
                .unwrap_or_default();

            for m in outgoing {
                if let Some(node) = self.nodes.get_mut(&m) {
                    node.remove_incoming(n);
                    if node.incoming_degree() == 0 {
                        no_incoming.push_back(m);
                    }
                }
            }
        }

        if result.len() == total_nodes {
            Ok(result)
        } else {
            Err("Cycle detected in graph")
        }
    }
}

impl DirectedGraph {
    fn from_update_and_rules(update: &Update, rules: &[Rule]) -> Self {
        let mut graph = DirectedGraph::new();

        for &page in &update.pages {
            graph.add_node(page);
        }

        for rule in rules.iter().filter(|r| r.apply_to(update)) {
            graph.add_edge(rule.left(), rule.right());
        }

        graph
    }
}

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
            let mut graph = DirectedGraph::from_update_and_rules(&update, &rules);
            let reordered_pages = graph.topological_sort().expect("Invalid graph structure");

            update.set_pages(reordered_pages)
        })
        .map(|update| *update.mid().unwrap())
        .sum();

    println!("{total:#?}");
}
