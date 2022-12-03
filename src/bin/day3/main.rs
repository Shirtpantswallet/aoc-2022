#![feature(array_chunks)]
use std::{collections::HashSet, fs};
fn main() {
    let contents = fs::read_to_string("./src/bin/day3/input.txt")
        .expect("Should have been able to read the file");
    let wonky_remapper = vec![
        27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
        50, 51, 52, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
        18, 19, 20, 21, 22, 23, 24, 25, 26,
    ];
    let part_1: usize = contents
        .split_whitespace()
        .into_iter()
        .map(|r| {
            let mut r = r.to_string();
            let unique_b = r
                .split_off((r.len() + 1) / 2)
                .chars()
                .collect::<HashSet<_>>();
            let result = r
                .chars()
                .filter_map(|c| match unique_b.contains(&c) {
                    true => Some(c),
                    false => None,
                })
                .collect::<HashSet<_>>();
            assert_eq!(result.len(), 1);

            wonky_remapper[*result.iter().next().unwrap() as usize - 65]
        })
        .sum();
    println!("Part 1: {:#?}", part_1);

    let part_2 = contents
        .split_whitespace()
        .collect::<Vec<_>>()
        .array_chunks_mut::<3>()
        .map(|v| {
            let mut v: Vec<HashSet<_>> = v
                .into_iter()
                .map(|s| s.chars().collect::<HashSet<_>>())
                .collect();
            assert_eq!(v.len(), 3);

            let (intersection, others) = v.as_mut_slice().split_at_mut(1);
            let intersection = &mut intersection[0];
            for other in others {
                intersection.retain(|e| other.contains(e));
            }
            assert_eq!(intersection.len(), 1);

            wonky_remapper[*intersection.iter().next().unwrap() as usize - 65]
        })
        .sum::<usize>();
    println!("Part 2: {:#?}", part_2);
}
