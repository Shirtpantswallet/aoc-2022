#![feature(iter_array_chunks)]
use std::collections::HashSet;

use regex::Regex;
static CONTENTS: &str = include_str!("input.txt");

fn overlaps_from_str(line: &str) -> Option<bool> {
    // let rg = Regex::new(r"(?P<low_1>\d+)-(?P<high_1>\d+),(?P<low_2>\d+)-(?P<high_2>\d+)");
    // line.map();
    todo!()
}
fn main() {
    // let contents = fs::read_to_string("./src/bin/day3/input.txt")
    //     .expect("Should have been able to read the file");

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
}
