use pathfinding::directed::topological_sort::topological_sort;
use regex::Regex;
use std::collections::{HashMap, HashSet};

type Bags = HashMap<String, Vec<InnerBag>>;

#[derive(Debug, Clone)]
pub struct InnerBag {
    name: String,
    count: usize,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String, Vec<InnerBag>> {
    let regex_outer_bag = Regex::new(r"^(.*) bags contain (.*)$").unwrap();
    let regex_inner_bags = Regex::new(r"(\d+) (.*?) bag").unwrap();

    input
        .lines()
        .map(|l| {
            (
                regex_outer_bag.captures(l).unwrap()[1].to_string(),
                regex_inner_bags
                    .captures_iter(l)
                    .map(|c| InnerBag {
                        name: c[2].to_string(),
                        count: c[1].parse().unwrap(),
                    })
                    .collect(),
            )
        })
        .collect()
}

fn sort_rev(bags: &Bags) -> Vec<&str> {
    let mut sorted = topological_sort(&bags.keys().map(|b| b.as_str()).collect::<Vec<_>>(), |&c| {
        bags[c].iter().map(|d| d.name.as_str())
    })
    .unwrap();
    sorted.reverse();
    sorted
}

#[aoc(day7, part1)]
pub fn solve_part_1(input: &Bags) -> usize {
    let mut shinygoldhs = HashSet::new();
    shinygoldhs.insert("shiny gold");

    for bi in sort_rev(input).iter() {
        if input[*bi]
            .iter()
            .any(|d| shinygoldhs.contains(d.name.as_str()))
        {
            shinygoldhs.insert(bi);
        }
    }

    shinygoldhs.len() - 1
}

#[aoc(day7, part2)]
pub fn solve_part_2(input: &Bags) -> usize {
    let mut contains: HashMap<&str, HashMap<&str, usize>> = HashMap::new();
    for c in sort_rev(input).into_iter() {
        let mut inside = HashMap::new();
        for ib in input[c].iter() {
            *inside.entry(ib.name.as_str()).or_insert(0) += ib.count;
            for (&dd, nn) in contains[&ib.name.as_str()].iter() {
                *inside.entry(dd).or_insert(0) += ib.count * nn;
            }
        }
        contains.insert(c, inside.into_iter().collect());
    }
    contains["shiny gold"].iter().map(|(_, &n)| n).sum()
}
