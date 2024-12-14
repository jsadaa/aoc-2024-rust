use std::collections::{HashMap, HashSet, VecDeque};

/// A directed graph implementation using adjacency lists
#[derive(Debug)]
struct DirectedGraph {
    nodes: HashMap<i32, Node>,
}

impl DirectedGraph {
    /// Creates a new empty directed graph
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    /// Adds a new node with the given value to the graph
    fn add_node(&mut self, value: i32) {
        self.nodes.insert(value, Node::new(value));
    }

    /// Adds a directed edge from one node to another
    fn add_edge(&mut self, from: i32, to: i32) {
        if let Some(from_node) = self.nodes.get_mut(&from) {
            from_node.add_outgoing(to);
        }
        if let Some(to_node) = self.nodes.get_mut(&to) {
            to_node.add_incoming(from);
        }
    }

    /// Performs a topological sort of the graph using Kahn's algorithm
    /// Returns the sorted nodes or an error if a cycle is detected
    fn topological_sort(&mut self) -> Result<Vec<i32>, &'static str> {
        let mut result = Vec::new();
        let total_nodes = self.nodes.len();

        // Get nodes with no incoming edges to start
        let mut no_incoming: VecDeque<i32> = self
            .nodes
            .values()
            .filter(|node| node.incoming_degree() == 0)
            .map(|node| node.value)
            .collect();

        // While there are nodes with no incoming edges
        while let Some(n) = no_incoming.pop_front() {
            result.push(n);

            // Get outgoing edges before modifying the graph
            let outgoing: Vec<i32> = self
                .nodes
                .get(&n)
                .map(|node| node.outgoing.iter().copied().collect())
                .unwrap_or_default();

            // Remove edges and check for new nodes with no incoming edges
            for m in outgoing {
                if let Some(node) = self.nodes.get_mut(&m) {
                    node.remove_incoming(n);
                    if node.incoming_degree() == 0 {
                        no_incoming.push_back(m);
                    }
                }
            }
        }

        // If we visited all nodes, sort succeeded, otherwise there was a cycle
        if result.len() == total_nodes {
            Ok(result)
        } else {
            Err("Cycle detected in graph")
        }
    }
}

impl DirectedGraph {
    /// Creates a new directed graph from an update and set of rules
    fn from_update_and_rules(update: &Update, rules: &[Rule]) -> Self {
        let mut graph = DirectedGraph::new();

        // Add nodes for each page
        for &page in &update.pages {
            graph.add_node(page);
        }

        // Add edges for each applicable rule
        for rule in rules.iter().filter(|r| r.apply_to(update)) {
            graph.add_edge(rule.left(), rule.right());
        }

        graph
    }
}

/// Represents an ordering rule between two pages
#[derive(Debug)]
struct Rule {
    left: i32,
    right: i32,
}

impl Rule {
    /// Creates a new rule specifying that left should come before right
    fn new(left: i32, right: i32) -> Self {
        Self { left, right }
    }

    /// Returns the left (source) page of the rule
    fn left(&self) -> i32 {
        self.left
    }

    /// Returns the right (destination) page of the rule
    fn right(&self) -> i32 {
        self.right
    }

    /// Checks if this rule applies to the given update
    fn apply_to(&self, update: &Update) -> bool {
        update.pages.contains(&self.left) && update.pages.contains(&self.right)
    }
}

/// Represents an update containing an ordered list of pages
#[derive(Debug)]
struct Update {
    pages: Vec<i32>,
}

impl Update {
    /// Creates a new update with the given pages
    fn new(pages: Vec<i32>) -> Self {
        Self { pages }
    }

    /// Checks if this update respects a given rule's ordering
    fn respect(&self, rule: &Rule) -> bool {
        self.pages.iter().position(|&p| p == rule.left()).unwrap()
            < self.pages.iter().position(|&p| p == rule.right()).unwrap()
    }

    /// Returns the middle element of the pages list
    fn middle_el(&self) -> Option<&i32> {
        self.pages.get(self.pages.len() / 2)
    }

    /// Updates the pages list and returns self
    fn set_pages(mut self, pages: Vec<i32>) -> Self {
        self.pages = pages;
        self
    }
}

/// Represents a node in the directed graph
#[derive(Debug, Clone)]
struct Node {
    value: i32,
    outgoing: HashSet<i32>,
    incoming: HashSet<i32>,
}

impl Node {
    /// Creates a new node with the given value
    fn new(value: i32) -> Self {
        Self {
            value,
            outgoing: HashSet::new(),
            incoming: HashSet::new(),
        }
    }

    /// Adds an incoming edge from the given node
    fn add_incoming(&mut self, from_value: i32) {
        self.incoming.insert(from_value);
    }

    /// Adds an outgoing edge to the given node
    fn add_outgoing(&mut self, to_value: i32) {
        self.outgoing.insert(to_value);
    }

    /// Removes an incoming edge from the given node
    fn remove_incoming(&mut self, from_value: i32) {
        self.incoming.remove(&from_value);
    }

    /// Removes an outgoing edge to the given node
    fn remove_outgoing(&mut self, to_value: i32) {
        self.outgoing.remove(&to_value);
    }

    /// Returns the number of incoming edges
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
        .map(|update| *update.middle_el().unwrap())
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
        .map(|update| *update.middle_el().unwrap())
        .sum();

    println!("Total = {total:#?}");
}
