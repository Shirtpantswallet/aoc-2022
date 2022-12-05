#![feature(iter_array_chunks)]

use std::collections::VecDeque;
static CONTENTS: &str = include_str!("input.txt");

fn main() {
    let mut input = CONTENTS.split("\n");

    let mut stacks = input
        .by_ref()
        // Empty line between container block and instruction block
        .take_while(|l| l.len() != 0)
        // Skip the first character, which is either a ' ' or a '[',
        // then step by 4, which is skips the '   ' or '] [' between
        // interesting characters
        .flat_map(|l| {
            l.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter_map(|(idx, c)| match c {
                    ' ' => None,
                    character => Some((idx, character)),
                })
        })
        // Arguably, this '9' is a magic number, but my brain is too
        // tired to figure out how to factor it out
        .fold(vec![VecDeque::default(); 9], |mut v, (idx, character)| {
            // the 1-based container stack index will be the last entry
            // Don't add it. Also ignore non-existent containers
            if character.is_ascii_alphabetic() {
                // Input is parsed top to bottom, but stacks are
                // built bottom to top
                v[idx].push_front(character);
            }
            v
        });
    println!("{:?}", stacks);

    // Although the instructions are the same for parts 1 and 2,
    // the interpretation is different for each. At least we can
    // loop through them all at once!
    let instructions = input
        .flat_map(|line| {
            // Skip "move", then take \d+, skip "from", take \d+, skip 2, and take final \d+
            line.split_whitespace()
                .skip(1)
                .step_by(2)
                .map(|c| c.parse::<usize>().unwrap())
        }) // Consume [count, source, destination] at a time
        .array_chunks::<3>();

    // I don't think there's a way to mutate them both without cloning
    let mut stacks_part_1 = stacks.clone();

    instructions.for_each(|[count, source, destination]| {
        let mut temp = VecDeque::new();
        // I tried drain(count..) as an alternative, but it panics a lot
        // let mut fofi_containers = stacks_part_1[source - 1].drain(count..).collect();
        // stacks_part_1[destination - 1].append(&mut fofi_containers);

        // let mut same_order_containers = stacks[source - 1].drain(count..).collect();
        // stacks[destination - 1].append(&mut same_order_containers);

        for _ in 0..count {
            if let Some(container) = stacks_part_1[source - 1].pop_back() {
                stacks_part_1[destination - 1].push_back(container);
            }
            if let Some(container) = stacks[source - 1].pop_back() {
                temp.push_front(container);
            }
        }
        stacks[destination - 1].append(&mut temp);
    });
    let part_1 = stacks_part_1
        .iter_mut()
        .filter_map(|stack| stack.back())
        .collect::<String>();
    let part_2 = stacks
        .iter_mut()
        .filter_map(|stack| stack.back())
        .collect::<String>();
    println!("{:#?}", part_1);
    println!("{:#?}", part_2);
}
