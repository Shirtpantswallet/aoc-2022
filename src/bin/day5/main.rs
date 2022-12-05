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
        // container characters
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
            // the 1-based container stack index will be the last entry.
            // Don't add it. Also ignore non-existent containers
            if character.is_ascii_alphabetic() {
                // Input is parsed top to bottom, but stacks are
                // built bottom to top
                v[idx].push_front(character);
            }
            v
        });

    // I don't think there's a way to mutate them both without cloning
    let mut stacks_part_1 = stacks.clone();

    // Although the instructions are the same for parts 1 and 2,
    // the interpretation is different for each. At least we can
    // loop through them all at once!
    for [count, source, destination] in input
        .flat_map(|line| {
            // Skip "move", then take \d+, skip "from", take \d+, skip 2, and take final \d+
            line.split_whitespace()
                .skip(1)
                .step_by(2)
                .map(|c| c.parse::<usize>().unwrap())
        })
        .array_chunks::<3>()
    {
        // Although we're only looking at the length of stacks_part_1[source-1], because
        // both parts 1 and 2 begin with the same container arrangement and use the same
        // instructions, the actual count of containers will be the same
        let actual_count = stacks_part_1[source - 1].len().saturating_sub(count);

        // FOFI -> First Out First In
        let fofi_containers: VecDeque<_> = stacks_part_1[source - 1]
            .drain(actual_count..)
            .rev()
            .collect();
        stacks_part_1[destination - 1].append(&mut fofi_containers.into());

        let same_order_containers: VecDeque<_> = stacks[source - 1].drain(actual_count..).collect();
        stacks[destination - 1].append(&mut same_order_containers.into());
    }

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
