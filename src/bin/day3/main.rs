#![feature(array_chunks)]
// use std::fs;

static CONTENTS: &str = include_str!("input.txt");
pub fn compartments_from_str(line: &str) -> [usize; 2] {
    let middle = line.len() / 2;

    let mut values = line.chars().map(|c| {
        let c = c as usize;
        if c > 90 {
            c - 96 // 'a' is Ordinal 97
        } else {
            c - 38 // 'A' is ordinal 65, but needs to be remapped to 27. 65 - 27 = 38
        }
    });

    [
        values
            .by_ref()
            .take(middle)
            .fold(0, |acc, x| acc | (1 << x)),
        values.fold(0, |acc, x| acc | (1 << x)),
    ]
}

fn main() {
    // let contents = fs::read_to_string("./src/bin/day3/input.txt")
    //     .expect("Should have been able to read the file");

    let (part_1, part_2) = CONTENTS
        .split_whitespace()
        .map(|r| compartments_from_str(r))
        .collect::<Vec<_>>()
        .array_chunks_mut::<3>()
        .map(|v| {
            let item_value_sum_3: usize = v
                .iter()
                .map(|r| {
                    let mask = r[0] & r[1];
                    mask.ilog2() as usize
                })
                .sum();

            let badge_value_sum = v
                .iter()
                .map(|r| r[0] | r[1])
                .reduce(|acc, r| acc & r)
                .unwrap()
                .ilog2() as usize;
            (item_value_sum_3, badge_value_sum)
        })
        .fold((0, 0), |(acc_item, acc_badge), (items, badge)| {
            (acc_item + items, acc_badge + badge)
        });
    println!("Part 1: {:#?}", part_1);
    println!("Part 2: {:#?}", part_2);
}
