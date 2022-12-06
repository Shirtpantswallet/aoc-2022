static CONTENTS: &str = include_str!("input.txt");

fn main() {
    let packet_length = 4;
    let part_1 = find_index(CONTENTS, packet_length).unwrap();

    println!("Part 1: {part_1}");
    // let message_length = 14;
    // let part_2 = find_index(CONTENTS, message_length).unwrap();
    // println!("Part 2: {part_2}");
}

pub fn find_index(input: &str, length: usize) -> Option<usize> {
    let input = input.as_bytes();
    // Index is the trailing character of the message, as the instructions
    // want to know how many total characters have been processed by the
    // *completion* of the first start-of-packet marker
    let mut index = length - 1;
    while index < input.len() {
        let mut inner_index = 1 + index - length;
        let mut bitarray: usize = 0;
        // while no collision of characters
        'inner: while (bitarray & 1 << (input[inner_index] - 'a' as u8)) == 0 {
            // println!("{inner_index}");
            bitarray ^= 1 << (input[inner_index] - 'a' as u8);
            if inner_index == index {
                let mut counter = 0; // c accumulates the total bits set in v
                while counter < bitarray {
                    bitarray &= bitarray - 1; // clear the least significant bit set
                    counter += 1;
                }
                if counter == length {
                    return Some(index + 1);
                } else {
                    break 'inner;
                }
            } else {
                inner_index += 1;
            }
        }
        index += std::cmp::max(index - inner_index, 1);
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_packet_1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let index = find_index(input, 4).unwrap();
        assert_eq!(index, 5);
    }

    #[test]
    fn test_packet_2() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let index = find_index(input, 4).unwrap();
        assert_eq!(index, 6);
    }
    #[test]
    fn test_message_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let index = find_index(input, 14).unwrap() - 9;
        assert_eq!(index, 19);
    }

    #[test]
    fn test_message_2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let index = find_index(input, 14).unwrap();
        assert_eq!(index, 23);
        // nppdvjthqldpwncqszvftbrmjlhg
        // nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
        // zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw
    }
}
