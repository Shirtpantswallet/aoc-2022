#![feature(array_chunks)]
use std::fs;

// static CONTENTS: &str = include_str!("input.txt");
pub fn compartments_from_str(line: &str) -> [usize; 2] {
    let wonky_remapper = vec![
        27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
        50, 51, 52, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
        18, 19, 20, 21, 22, 23, 24, 25, 26,
    ];

    let mut compartment_1 = 0;
    let mut compartment_2 = 0;

    let (first_half, second_half) = line.split_at(line.len() / 2);

    for index in first_half.chars().map(|c| wonky_remapper[c as usize - 65]) {
        compartment_1 = compartment_1 | 1 << index;
    }
    for index in second_half.chars().map(|c| wonky_remapper[c as usize - 65]) {
        compartment_2 = compartment_2 | 1 << index;
    }
    [compartment_1, compartment_2]
}

fn main() {
    for _ in 0..10000 {
        let contents = fs::read_to_string("./src/bin/day3/input.txt")
            .expect("Should have been able to read the file");

        let (part_1, part_2) = contents
            .split_whitespace()
            .map(|r| compartments_from_str(r))
            .collect::<Vec<_>>()
            .array_chunks_mut::<3>()
            .map(|v| {
                let item_value_sum_3: usize = v
                    .iter()
                    .map(|r| {
                        let mask = r[0] & r[1];
                        for counter in 1..53 {
                            if mask == (1 << counter) {
                                return counter;
                            }
                        }
                        panic!("The item number should have been found");
                    })
                    .sum();

                let badge_value_mask =
                    (v[0][0] | v[0][1]) & (v[1][0] | v[1][1]) & (v[2][0] | v[2][1]);
                let mut badge_value_sum = 0;
                for counter in 1..53 {
                    if badge_value_mask == (1 << counter) {
                        badge_value_sum = counter;
                        break;
                    }
                }
                (item_value_sum_3, badge_value_sum)
            })
            .reduce(|(acc_item, acc_badge), (items, badge)| (acc_item + items, acc_badge + badge))
            .unwrap();
        println!("Part 1: {:#?}", part_1);
        println!("Part 2: {:#?}", part_2);
    }
}
