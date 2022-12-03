#![feature(array_chunks)]
use bitvec::prelude::*;
use std::{collections::HashSet, fs};

#[derive(Debug)]
struct Rucksack {
    compartment_1: BitArray,
    compartment_2: BitArray,
}

impl Rucksack {
    pub fn new(compartment_1: BitArray, compartment_2: BitArray) -> Self {
        assert_eq!(compartment_1.len(), compartment_2.len());
        Self {
            compartment_1,
            compartment_2,
        }
    }

    pub fn from_str(line: &str) -> Self {
        let wonky_remapper = vec![
            27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
            49, 50, 51, 52, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
        ];

        let mut compartment_1 = bitarr![0; 52];
        let mut compartment_2 = bitarr![0; 52];

        let (first_half, second_half) = line.split_at(line.len() / 2);

        for index in first_half.chars().map(|c| wonky_remapper[c as usize - 65]) {
            compartment_1.set(index, true);
        }
        for index in second_half.chars().map(|c| wonky_remapper[c as usize - 65]) {
            compartment_2.set(index, true);
        }
        Self::new(compartment_1, compartment_2)
    }
    pub fn contents(&self) -> BitVec {
        self.compartment_1
            .iter()
            .by_vals()
            .zip(self.compartment_2.iter().by_vals())
            .map(|(x1, x2)| x1 || x2)
            .collect::<BitVec>()
    }

    pub fn find_duplicate_item_value(&self) -> usize {
        self.compartment_1
            .iter()
            .by_vals()
            .zip(self.compartment_2.iter().by_vals())
            .enumerate()
            .map(|(e, (x1, x2))| if x1 && x2 { e } else { 0 })
            .sum::<usize>()
    }
}

fn find_group_badge_value(group: &[Rucksack; 3]) -> usize {
    let badge_value_sum = group[0]
        .contents()
        .iter()
        .by_vals()
        .zip(group[1].contents().iter().by_vals())
        .zip(group[2].contents().iter().by_vals())
        .map(|((x1, x2), x3)| (x1, x2, x3))
        .enumerate()
        .map(|(e, (x1, x2, x3))| if x1 && x2 && x3 { e } else { 0 })
        .sum::<usize>();
    badge_value_sum
}

fn main() {
    let contents = fs::read_to_string("./src/bin/day3/input.txt")
        .expect("Should have been able to read the file");

    let (part_1, part_2) = contents
        .split_whitespace()
        .map(|r| Rucksack::from_str(r))
        .collect::<Vec<Rucksack>>()
        .array_chunks_mut::<3>()
        .map(|v| {
            let item_value_sum_3: usize = v.iter().map(|r| r.find_duplicate_item_value()).sum();

            let badge_value_sum = find_group_badge_value(&v);
            (item_value_sum_3, badge_value_sum)
        })
        .reduce(|(acc_item, acc_badge), (items, badge)| (acc_item + items, acc_badge + badge))
        .unwrap();
    println!("Part 1: {:#?}", part_1);
    println!("Part 2: {:#?}", part_2);
}
