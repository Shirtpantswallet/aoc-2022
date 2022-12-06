static CONTENTS: &str = include_str!("input.txt");

fn main() {
    let datastream: &[u8] = CONTENTS.as_bytes();

    // Index is the trailing character of the message, as the instructions
    // want to know how many total characters have been processed by the
    // *completion* of the first start-of-packet marker
    let mut index = 3;
    while index < datastream.len() {
        if let [first, second, third, fourth] = datastream[(index - 3)..=index] {
            if third == fourth {
                index += 3;
            } else if second == third {
                index += 2;
            } else if first == second {
                index += 1;
            } else {
                if first != third && first != fourth && second != fourth {
                    break;
                }
                index += 1;
            }
        }
    }
    let index = index + 1; // 1-based indexes
    println!("Part 1: {index}");
}
