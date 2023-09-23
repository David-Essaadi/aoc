use core::fmt;
use std::cmp::Ordering;

fn main() {
    solution_1(&lib::read_file_contents("inputs/input-2.txt"));
    solution_2(&lib::read_file_contents("inputs/input-2.txt"));
}

fn solution_1(contents: &String) -> i32 {
    let rounds = contents.trim().split("\n");
    rounds.map(|r| points_for_round(&parse_round(&r))).reduce(|acc, curr| acc + curr).unwrap_or(0)
}

fn solution_2(contents: &String) -> i32 {
    let rounds = contents.trim().split("\n");
    rounds.map(|r| points_for_round(&parse_round_2(&r))).reduce(|acc, curr| acc + curr).unwrap_or(0)
}

#[derive(Clone, Copy, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (RPS::Rock, RPS::Rock) => Some(Ordering::Equal),
            (RPS::Rock, RPS::Paper) => Some(Ordering::Less),
            (RPS::Rock, RPS::Scissors) => Some(Ordering::Greater),
            (RPS::Paper, RPS::Rock) => Some(Ordering::Greater),
            (RPS::Paper, RPS::Paper) => Some(Ordering::Equal),
            (RPS::Paper, RPS::Scissors) => Some(Ordering::Less),
            (RPS::Scissors, RPS::Rock) => Some(Ordering::Less),
            (RPS::Scissors, RPS::Paper) => Some(Ordering::Greater),
            (RPS::Scissors, RPS::Scissors) => Some(Ordering::Equal),
        }
    }
}

impl fmt::Display for RPS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RPS::Rock => write!(f, "rock"),
            RPS::Paper => write!(f, "paper"),
            RPS::Scissors => write!(f, "scissors"),
        }
    }
}

#[derive(PartialEq)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Outcome::Loss => write!(f, "They win"),
            Outcome::Draw => write!(f, "It's a draw"),
            Outcome::Win => write!(f, "We win"),
        }
    }
}

struct Round(RPS, RPS);
impl std::fmt::Display for Round {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "They play {} and we play {}.", self.0, self.1)
    }
}

fn outcome_of_round(round: &Round) -> Outcome {
    match round {
        Round(x, y) if x < y => Outcome::Win,
        Round(x, y) if x == y => Outcome::Draw,
        _ => Outcome::Loss,
    }
}

fn points_for_played(played: &RPS) -> i32{
    match played {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    }
}

fn points_for_round(round: &Round) -> i32 {
    points_for_played(&round.1) + match outcome_of_round(round) {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

fn parse_round(line: &str) -> Round {
    match line {
        "A X" => Round(RPS::Rock, RPS::Rock),
        "A Y" => Round(RPS::Rock, RPS::Paper),
        "A Z" => Round(RPS::Rock, RPS::Scissors),
        "B X" => Round(RPS::Paper, RPS::Rock),
        "B Y" => Round(RPS::Paper, RPS::Paper),
        "B Z" => Round(RPS::Paper, RPS::Scissors),
        "C X" => Round(RPS::Scissors, RPS::Rock),
        "C Y" => Round(RPS::Scissors, RPS::Paper),
        "C Z" => Round(RPS::Scissors, RPS::Scissors),
        _ => panic!("invalid input: {}", line)
    }
}

fn find_required_move(opponent: &RPS, desired_outcome: Outcome) -> Round {
    let all_choices = [RPS::Rock, RPS::Paper, RPS::Scissors];

    for &choice in all_choices.iter() {
        let round = Round(*opponent, choice);
        if outcome_of_round(&round) == desired_outcome {
            return round
        }
    }

    panic!("unable to find required move")
}

fn parse_round_2(line: &str) -> Round {
    find_required_move(&match line.chars().nth(0).unwrap() {
        'A' => RPS::Rock,
        'B' => RPS::Paper,
        'C' => RPS::Scissors,
        _ => panic!("incorrect input")
    }, match line.chars().nth(2).unwrap() {
        'X' => Outcome::Loss,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!("incorrect input")
    })
}

#[cfg(test)]
mod tests {
    use std::assert_eq;
    use lib::read_file_contents;

    use super::*;

    #[test]
    fn test_solution_1_example() {
        let contents = read_file_contents("inputs/example-2.txt");
        assert_eq!(solution_1(&contents), 15);
    }

    #[test]
    fn test_solution_1() {
        let contents = read_file_contents("inputs/input-2.txt");
        assert_eq!(solution_1(&contents), 10941);
    }

    #[test]
    fn test_solution_2_example() {
        let contents = read_file_contents("inputs/example-2.txt");
        assert_eq!(solution_2(&contents), 12);
    }
    
    #[test]
    fn test_solution_2() {
        let contents = read_file_contents("inputs/input-2.txt");
        assert_eq!(solution_2(&contents), 13071);
    }
}
