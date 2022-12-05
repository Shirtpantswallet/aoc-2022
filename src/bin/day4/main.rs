#![feature(iter_array_chunks)]
use std::cmp::Ordering;

static CONTENTS: &str = include_str!("input.txt");

enum RangeOverlap {
    None,
    Intersection,
    Contained,
}

fn main() {
    for _ in 0..1000 {
        let (part_1, part_2) = CONTENTS
            .split_ascii_whitespace()
            .flat_map(|line| line.split(",").flat_map(|range| range.split("-")))
            .map(|bound| bound.parse::<u8>().unwrap())
            .array_chunks::<4>()
            .map(|v| {
                if v[1] < v[2] || v[0] > v[3] {
                    RangeOverlap::None
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
}
