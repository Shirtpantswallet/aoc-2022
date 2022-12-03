#![feature(array_chunks)]
use std::fs;

// static CONTENTS: &str = include_str!("input.txt");
#[target_feature(enable = "avx2")]
pub unsafe fn compartments_from_str(line: &str) -> [usize; 2] {
    let (first_half, second_half) = line.split_at(line.len() / 2);
    let compartment_1 = first_half
        .chars()
        .map(|c| {
            let c = c as usize;
            if c > 90 {
                c - 96 // 'a' is Ordinal 97
            } else {
                c - 38 // 'A' is ordinal 65, but needs to be remapped to 27. 65 - 27 = 38
            }
        })
        .fold(0, |acc, x| acc | (1 << x));
    let compartment_2 = second_half
        .chars()
        .map(|c| {
            let c = c as usize;
            if c > 90 {
                c - 96 // 'a' is Ordinal 97
            } else {
                c - 38 // 'A' is ordinal 65, but needs to be remapped to 27. 65 - 27 = 38
            }
        })
        .fold(0, |acc, x| acc | (1 << x));
    [compartment_1, compartment_2]
}

fn main() {
    for _ in 0..10000 {
        let contents = fs::read_to_string("./src/bin/day3/input.txt")
            .expect("Should have been able to read the file");

        let (part_1, part_2) = contents
            .split_whitespace()
            .map(|r| unsafe { compartments_from_str(r) })
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
