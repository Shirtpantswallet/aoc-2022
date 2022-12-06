#![feature(iter_array_chunks)]

static CONTENTS: &str = include_str!("input.txt");

fn main() {
    let mut input = CONTENTS.split("\n");

    let stacks_diagram = input
        .by_ref()
        // Empty line between container block and instruction block
        .take_while(|l| l.len() != 0)
        .collect::<Vec<_>>()
        .drain(..)
        // The diagram is given top to bottom.
        // We want to parse it bottom to top.
        .rev()
        .collect::<Vec<_>>();

    let number_of_stacks = stacks_diagram
        .iter()
        .next()
        .unwrap()
        .chars()
        // Skip the first character, which is either a ' ' or a '[',
        // then step by 4, which is skips the '   ' or '] [' between
        // container characters
        .skip(1)
        .step_by(4)
        .collect::<Vec<_>>()
        .len();

    // Now that we've removed that stack IDs from the bottom, the total
    // number of rows in the stack diagram is the max stack height
    // at initialization.
    let _initial_stack_height = stacks_diagram.len();

    // This is the max stack height during runtime, determined below
    let experimental_stack_height = 47;

    let mut stacks = stacks_diagram
        .iter()
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
        .fold(
            vec![Vec::with_capacity(experimental_stack_height); number_of_stacks],
            |mut v, (idx, character)| {
                v[idx].push(character);
                v
            },
        );

    // I don't think there's a way to mutate them both without cloning
    let mut stacks_part_1 = stacks.clone();

    // let mut max_stack_height = 0;
    // Although the instructions are the same for parts 1 and 2,
    // the interpretation is different for each. At least we can
    // loop through them all at once!
    for (count, source, destination) in input
        .flat_map(|line| {
            // Skip "move", then take \d+, skip "from", take \d+, skip 2, and take final \d+
            line.split_whitespace()
                .skip(1)
                .step_by(2)
                .map(|c| c.parse::<usize>().unwrap())
        })
        .array_chunks::<3>()
        // Stack IDs are base 1. Correct offset to base 0.
        .map(|[count, source, destination]| (count, source - 1, destination - 1))
    {
        // Although we're only looking at the length of stacks_part_1[source-1], because
        // both parts 1 and 2 begin with the same container arrangement and use the same
        // instructions, the actual count of containers will be the same
        let actual_count = stacks_part_1[source].len().saturating_sub(count);

        // FOFI -> First Out First In
        let fofi_containers = stacks_part_1[source]
            .drain(actual_count..)
            .rev()
            .collect::<Vec<_>>();
        stacks_part_1[destination].append(&mut fofi_containers.into());

        let same_order_containers = stacks[source].drain(actual_count..).collect::<Vec<_>>();
        stacks[destination].append(&mut same_order_containers.into());
        // max_stack_height = std::cmp::max(
        //     max_stack_height,
        //     stacks.iter().map(|l| l.len()).max().unwrap(),
        // );
    }

    let part_1 = stacks_part_1
        .iter_mut()
        .filter_map(|stack| stack.pop())
        .collect::<String>();

    let part_2 = stacks
        .iter_mut()
        .filter_map(|stack| stack.pop())
        .collect::<String>();

    println!("{:#?}", part_1);
    println!("{:#?}", part_2);
    // println!("{}", max_stack_height);
}
