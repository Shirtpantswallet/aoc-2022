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

    let _instructions = input
        .flat_map(|line| {
            // Skip "move", then take \d+, skip "from", take \d+, skip 2, and take final \d+
            line.split_whitespace()
                .skip(1)
                .step_by(2)
                .map(|c| c.parse::<usize>().unwrap())
        }) // Consume [count, source, destination] at a time
        .array_chunks::<3>()
        .for_each(|[count, source, destination]| {
            // println!("moving {count} from  {source} to {destination}");
            for _ in 0..count {
                if let Some(container) = stacks[source - 1].pop_back() {
                    stacks[destination - 1].push_back(container);
                } else {
                    break;
                }
            }
        });
    let top_containers = stacks
        .iter()
        .filter_map(|stack| stack.back())
        .collect::<String>();
    println!("{:#?}", top_containers);
}
