static CONTENTS: &str = include_str!("input.txt");

pub fn find_index(input: &str, length: usize) -> Option<usize> {
    if length > input.len() || length == 0 {
        return None;
    }
    let input = input.as_bytes();
    'outer: for slow in 0..(input.len() - length) {
        let mut bitarray: usize = 0;
        for fast in slow..(slow + length) {
            if (bitarray & 1 << (input[fast] - 'a' as u8) as usize) != 0 {
                // Collisions always happen with with input[slow]
                continue 'outer;
            }
            bitarray ^= 1 << (input[fast] - 'a' as u8);
        }
        return Some(slow + length);
    }
    None
    // Now we know there is a collision, so let's backtrack to skip forward!
    // while input[slow] != input[fast] {
    //     // Unset the keys between slow..collision
    //     bitarray ^= 1 << (input[slow] - 'a' as u8);
    //     slow += 1;
    // }
    // // Begin next loop with collision+1
    // slow += 1;
}

fn main() {
    let packet_length = 4;
    let part_1 = find_index(CONTENTS, packet_length).unwrap();

    let message_length = 14;
    let part_2 = find_index(CONTENTS, message_length).unwrap();

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
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
    fn test_packet_3() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let index = find_index(input, 4).unwrap();
        assert_eq!(index, 10);
    }

    #[test]
    fn test_packet_4() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let index = find_index(input, 4).unwrap();
        assert_eq!(index, 11);
    }

    #[test]
    fn test_message_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let index = find_index(input, 14).unwrap();
        assert_eq!(index, 19);
    }

    #[test]
    fn test_message_2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let index = find_index(input, 14).unwrap();
        assert_eq!(index, 23);
    }
    #[test]
    fn test_message_3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let index = find_index(input, 14).unwrap();
        assert_eq!(index, 23);
    }
    #[test]
    fn test_message_4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let index = find_index(input, 14).unwrap();
        assert_eq!(index, 29);
    }
    #[test]
    fn test_message_5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let index = find_index(input, 14).unwrap();
        assert_eq!(index, 26);
    }
}
