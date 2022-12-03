use std::fs;

#[derive(PartialEq)]
enum Hand {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl Hand {
    fn get_elf_hand(hand: &str) -> Self {
        match hand {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!("Invalid hand"),
        }
    }

    fn get_your_hand(hand: &str) -> Self {
        match hand {
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            _ => panic!("Invalid hand"),
        }
    }

    fn get_result_score(elf: &Self, you: &Self) -> i32 {
        if elf == you {
            3
        } else {
            match (elf, you) {
                (Hand::Rock, Hand::Paper) => 6,
                (Hand::Rock, Hand::Scissors) => 0,
                (Hand::Paper, Hand::Scissors) => 6,
                (Hand::Paper, Hand::Rock) => 0,
                (Hand::Scissors, Hand::Rock) => 6,
                (Hand::Scissors, Hand::Paper) => 0,
                _ => panic!("Invalid game"),
            }
        }
    }

    fn get_shape_score(hand: &Self) -> i32 {
        match hand {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn get_winner(&self) -> Self {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    fn get_loser(&self) -> Self {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
}

fn main() {
    // read in input
    let file_string = fs::read_to_string("input").expect("File not found");
    let pairs = file_string
        .lines()
        .map(|x| x.split_whitespace().collect::<Vec<&str>>());

    let score = pairs
        .clone()
        .map(|x| (Hand::get_elf_hand(x[0]), Hand::get_your_hand(x[1])))
        .fold(0, |acc, hands| {
            acc + Hand::get_shape_score(&hands.1) + Hand::get_result_score(&hands.0, &hands.1)
        });
    println!("score = {}", score);

    let score2 = pairs
        .map(|x| match x[1] {
            "X" => (
                Hand::get_elf_hand(x[0]),
                Hand::get_elf_hand(x[0]).get_loser(),
            ),
            "Y" => (Hand::get_elf_hand(x[0]), Hand::get_elf_hand(x[0])),
            "Z" => (
                Hand::get_elf_hand(x[0]),
                Hand::get_elf_hand(x[0]).get_winner(),
            ),
            _ => panic!("Invalid result"),
        })
        .fold(0, |acc, hands| {
            acc + Hand::get_shape_score(&hands.1) + Hand::get_result_score(&hands.0, &hands.1)
        });
    println!("score2 = {}", score2);
}
