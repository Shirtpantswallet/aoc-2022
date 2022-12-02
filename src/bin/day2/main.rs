use std::fs;

#[derive(Clone, Copy, Debug)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

struct Round {
    own: HandShape,
    opponent: HandShape,
}

impl Round {
    fn new(own: &HandShape, opponent: &HandShape) -> Self {
        Self {
            own: own.clone(),
            opponent: opponent.clone(),
        }
    }
    fn from_line(line: &str) -> Option<Self> {
        println!("{}", line);
        let pair = line
            .split(" ")
            .filter_map(|c| match c {
                "A" | "X" => Some(HandShape::Rock),
                "B" | "Y" => Some(HandShape::Paper),
                "C" | "Z" => Some(HandShape::Scissors),
                _ => None,
            })
            .collect::<Vec<HandShape>>();
        match pair.as_slice() {
            [opponent, own] => Some(Self::new(own, opponent)),
            _ => None,
        }
    }
    fn calculate_points(&self) -> usize {
        // The score for a single round is the score for the shape
        // you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
        // plus the score for the outcome of the round
        // (0 if you lost, 3 if the round was a draw, and 6 if you won).
        match (self.own, self.opponent) {
            // shape + outcome
            (HandShape::Rock, HandShape::Paper) => 1, // 1 + 0
            (HandShape::Paper, HandShape::Scissors) => 2, // 2 + 0
            (HandShape::Scissors, HandShape::Rock) => 3, // 3 + 0
            (HandShape::Rock, HandShape::Rock) => 4,  // 1 + 3
            (HandShape::Paper, HandShape::Paper) => 5, // 2 + 3
            (HandShape::Scissors, HandShape::Scissors) => 6, // 3 + 3
            (HandShape::Rock, HandShape::Scissors) => 7, // 1 + 6
            (HandShape::Paper, HandShape::Rock) => 8, // 2 + 6
            (HandShape::Scissors, HandShape::Paper) => 9, // 3 + 6
        }
    }
}

fn main() {
    let contents = fs::read_to_string("./src/bin/day2/input.txt")
        .expect("Should have been able to read the file");

    let final_score: usize = contents
        .lines()
        .into_iter()
        .map(|l| match Round::from_line(l) {
            Some(round) => round.calculate_points(),
            None => 0,
        })
        .sum();

    println!("Part 1:");
    println!("Final score: {final_score}");
}
