#![feature(binary_heap_into_iter_sorted)]
use std::cmp::Ordering;
// use std::env;
use std::fs;
use std::collections::BinaryHeap;

struct Elf {
    pub index: usize,
    pub calories: usize
}

impl Eq for Elf {}

impl Ord for Elf {
    fn cmp(&self, other: &Elf) -> Ordering {
        self.calories.cmp(&other.calories)
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Elf) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Elf) -> bool {
        self.calories == other.calories
    }
}


fn main() {
    let contents = fs::read_to_string("./src/bin/day1/input.txt")
        .expect("Should have been able to read the file");

    let elf_calories = contents.split("\n\n")
        .into_iter()
        .map(|s_all| 
            s_all.split("\n")
                .map(|s| s.parse::<usize>().unwrap_or_default())
                .collect::<Vec<usize>>()
                .iter()
                .sum())
        .enumerate()
        .map(|(index, calories)|
            Elf { index, calories}
        )
        .collect::<BinaryHeap<Elf>>();
    
    let max_elf: &Elf = elf_calories.iter().take(1).collect::<Vec<&Elf>>().first().unwrap();
    let (max_index, max_calories) = (max_elf.index, max_elf.calories);

    println!("Part 1:");
    println!("Max elf index is {max_index} with {max_calories} calories");
    
    
    let max_top_3: usize = elf_calories
        .into_iter_sorted()
        .take(3)
        .map(|e| e.calories)
        .sum();
    
    println!("Part 2:");
    println!("Max of top 3 elves is {max_top_3} calories");
}
