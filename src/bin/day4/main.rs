#![feature(iter_array_chunks)]

static CONTENTS: &str = include_str!("input.txt");

enum RangeOverlap {
    None,
    Intersection,
    Contained,
}

fn main() {
    let (part_1, part_2) = CONTENTS
        .split_ascii_whitespace()
        .flat_map(|line| line.split(",").flat_map(|range| range.split("-")))
        .map(|bound| bound.parse::<usize>().unwrap())
        .array_chunks::<4>()
        .map(|v| {
            let [left_low, left_high, right_low, right_high] = v;
            if left_high < right_low || left_low > right_high {
                RangeOverlap::None
            } else if (left_low > right_low && left_high > right_high)
                || (left_low < right_low && left_high < right_high)
            {
                RangeOverlap::Intersection
            } else {
                RangeOverlap::Contained
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
