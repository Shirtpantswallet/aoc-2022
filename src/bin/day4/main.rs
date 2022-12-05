#![feature(iter_array_chunks)]
use regex::Regex;
use std::cmp::Ordering;

static CONTENTS: &str = include_str!("input.txt");

enum RangeOverlap {
    None,
    Intersection,
    Contained,
}

fn main() {
    let rg = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let (part_1, part_2) = rg
        .captures_iter(CONTENTS)
        .map(|c| {
            let v = c
                .iter()
                .skip(1)
                .map(|m| m.unwrap().as_str().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            assert_eq!(v.len(), 4);
            // A-B, C-D
            if v[1] < v[2] || v[3] < v[0] {
                RangeOverlap::None // fully disjoint
            } else {
                match (v[0].cmp(&v[2]), v[1].cmp(&v[3])) {
                    (Ordering::Greater, Ordering::Greater) => RangeOverlap::Intersection,
                    (Ordering::Less, Ordering::Less) => RangeOverlap::Intersection,
                    (_, _) => RangeOverlap::Contained,
                }
            }
        })
        .fold((0, 0), |(contained, intersections), item| match item {
            RangeOverlap::None => (contained, intersections),
            RangeOverlap::Intersection => (contained, intersections + 1),
            RangeOverlap::Contained => (contained + 1, intersections + 1),
        });
    println!("Part 1: {:#?}", part_1);
    println!("Part 2: {:#?}", part_2);
}
