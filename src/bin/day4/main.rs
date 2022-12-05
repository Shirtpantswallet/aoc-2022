#![feature(iter_array_chunks)]
use regex::Regex;
use std::collections::HashSet;

static CONTENTS: &str = include_str!("input.txt");

fn main() {
    let rg = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let part_1 = rg
        .captures_iter(CONTENTS)
        .map(|c| {
            c.iter()
                .skip(1)
                .filter_map(|m| m.unwrap().as_str().parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .map(|v| {
            assert_eq!(v.len(), 4);
            let set_1 = (v[0]..=v[1]).collect::<HashSet<_>>();
            let set_2 = (v[2]..=v[3]).collect::<HashSet<_>>();
            set_1.is_subset(&set_2) || set_1.is_superset(&set_2)
        })
        .fold(0, |acc, item| if item { acc + 1 } else { acc });
    println!("Part 1: {:#?}", part_1);
    let part_2 = rg
        .captures_iter(CONTENTS)
        .map(|c| {
            c.iter()
                .skip(1)
                .filter_map(|m| m.unwrap().as_str().parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .map(|v| {
            assert_eq!(v.len(), 4);
            let set_1 = (v[0]..=v[1]).collect::<HashSet<_>>();
            let set_2 = (v[2]..=v[3]).collect::<HashSet<_>>();
            !set_1.is_disjoint(&set_2)
        })
        .fold(0, |acc, item| if item { acc + 1 } else { acc });
    println!("Part 2: {:#?}", part_2);
}
