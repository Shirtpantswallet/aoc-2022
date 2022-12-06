static CONTENTS: &str = include_str!("input.txt");

fn main() {
    let datastream = CONTENTS.as_bytes();
    let leading_indexes = datastream
        .windows(4)
        .enumerate()
        .filter_map(|(index, window)| {
            if let [first, second, third, fourth] = window {
                if first != second
                    && first != third
                    && first != fourth
                    && second != third
                    && second != fourth
                    && third != fourth
                {
                    Some(index)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();
    let part_1 = leading_indexes.first().unwrap() + 4; // + 3 for the length, + 1 for 1-indexing
    println!("{part_1}");
}
