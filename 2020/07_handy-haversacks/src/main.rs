use helpers::input_lines;
use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Edge<ID> {
    weight: usize,
    id: ID,
}

#[derive(Debug, Default)]
struct Graph<ID> {
    nodes: HashMap<ID, Vec<Edge<ID>>>,
    reverse: HashMap<ID, Vec<Edge<ID>>>,
}

impl<ID: Clone + Eq + std::hash::Hash> Graph<ID> {
    fn add_edge(&mut self, src_node: ID, dst_node: ID, weight: usize) {
        if let Entry::Vacant(entry) = self.nodes.entry(src_node.clone()) {
            entry.insert(Vec::new());
        }
        if let Entry::Vacant(entry) = self.nodes.entry(dst_node.clone()) {
            entry.insert(Vec::new());
        }
        if let Entry::Vacant(entry) = self.reverse.entry(src_node.clone()) {
            entry.insert(Vec::new());
        }
        if let Entry::Vacant(entry) = self.reverse.entry(dst_node.clone()) {
            entry.insert(Vec::new());
        }
        self.nodes.get_mut(&src_node).unwrap().push(Edge {
            weight,
            id: dst_node.clone(),
        });
        self.reverse.get_mut(&dst_node).unwrap().push(Edge {
            weight,
            id: src_node,
        });
    }

    fn get_edges(&self, node: &ID) -> &[Edge<ID>] {
        self.nodes.get(node).unwrap().as_slice()
    }

    fn get_reverse_edges(&self, node: &ID) -> &[Edge<ID>] {
        self.reverse.get(node).unwrap().as_slice()
    }
}

type BagRules<'a> = Graph<&'a str>;

fn load_bag_rules(lines: &[String]) -> BagRules {
    let line_re: Regex = Regex::new(r"^(.*?) bags contain (.*)\.$").unwrap();
    let item_re: Regex = Regex::new("^(\\d+) (.*?) bags?$").unwrap();

    let mut graph = Graph::default();
    for line in lines {
        let line_captured = line_re.captures(line).unwrap();

        let container_color = line_captured.get(1).unwrap().as_str();

        let rules = line_captured.get(2).unwrap().as_str();
        for rule in rules.split(", ") {
            if rule != "no other bags" {
                let rule_captured = item_re.captures(rule).unwrap();
                let contained_color = rule_captured.get(2).unwrap().as_str();
                let count: usize = rule_captured.get(1).unwrap().as_str().parse().unwrap();
                graph.add_edge(contained_color, container_color, count);
            }
        }
    }
    graph
}

fn part01(rules: &BagRules) -> usize {
    const TARGET_COLOR: &str = "shiny gold";

    let mut checked_colors = HashSet::<&str>::new();
    let mut colors_to_check = HashSet::<&str>::new();
    colors_to_check.insert(TARGET_COLOR);

    while !colors_to_check.is_empty() {
        let color = *colors_to_check.iter().next().unwrap();
        colors_to_check.remove(color);
        for edge in rules.get_edges(&color) {
            if !checked_colors.contains(edge.id) {
                colors_to_check.insert(edge.id);
            }
        }

        checked_colors.insert(color);
    }
    checked_colors.len() - 1
}

fn part02(rules: &BagRules) -> usize {
    const START_COLOR: &str = "shiny gold";

    fn bags_to_add<'a>(rules: &'a BagRules, color: &'a str) -> impl Iterator<Item = &'a str> {
        rules
            .get_reverse_edges(&color)
            .iter()
            .flat_map(|edge| vec![edge.id; edge.weight])
    }

    let mut bag_to_check = 0usize;
    let mut bags = bags_to_add(rules, START_COLOR).collect::<Vec<_>>();

    while bag_to_check != bags.len() {
        let bag = bags[bag_to_check];
        bags.extend(bags_to_add(rules, bag));
        bag_to_check += 1;
    }
    bags.len()
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    let bag_rules = load_bag_rules(&lines);

    println!("Part 1: {}", part01(&bag_rules));
    println!("Part 2: {}", part02(&bag_rules));
    Ok(())
}
