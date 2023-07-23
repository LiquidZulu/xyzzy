/*
 * https://adventofcode.com/2022/day/2
 * - RPS torunament alreaddy underway
 * - R > S; S > P; P > R; Same shape means draw
 * - opponent:
 * - - A=Rock
 * - - B=Paper
 * - - C=Scissors
 * - me:
 * - - X=Rock
 * - - Y=Paper
 * - - Z=Scissors
 * - Total score is the sum of scores for each round
 * - Each round is scored as SHAPE + OUTCOME
 * - - SHAPE { Rock=1; Paper=2; Scissors=3 }
 * - - OUTCOME { loss=0; draw=3; win=6 }
 */

use std::fs;

enum EPlayer {
    Opponent,
    Me,
}
use EPlayer::*;

enum Choice {
    Rock,
    Paper,
    Scissors,
    ChoiceUnknown,
    NeedWin,
    NeedLose,
    NeedDraw,
}
use Choice::*;

enum Outcome {
    Loss,
    Draw,
    Win,
    OutcomeUnknown,
}
use Outcome::*;

fn get_outcome(opponents_choice: &Choice, my_choice: &Choice, player: &EPlayer) -> Outcome {
    match (opponents_choice, my_choice) {
        (Rock, Rock) => Draw,
        (Paper, Paper) => Draw,
        (Scissors, Scissors) => Draw,
        (Rock, Paper) => match player {
            Opponent => Loss,
            Me => Win,
        },
        (Rock, Scissors) => match player {
            Opponent => Win,
            Me => Loss,
        },
        (Paper, Rock) => match player {
            Opponent => Win,
            Me => Loss,
        },
        (Paper, Scissors) => match player {
            Opponent => Loss,
            Me => Win,
        },
        (Scissors, Rock) => match player {
            Opponent => Loss,
            Me => Win,
        },
        (Scissors, Paper) => match player {
            Opponent => Win,
            Me => Loss,
        },
        _ => {
            println!("Does rust not have union types?");
            OutcomeUnknown
        }
    }
}

fn outcome_to_int(outcome: Outcome) -> i32 {
    match outcome {
        Loss => 0,
        Draw => 3,
        Win => 6,
        OutcomeUnknown => -69420,
    }
}

fn choice_to_int(choice: &Choice) -> i32 {
    match choice {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
        ChoiceUnknown => -69420,
        _ => {
            println!("this should literally never print. I am currently at choice_to_int");
            -999999
        }
    }
}

fn score_round_part_1(round: Vec<Choice>, player: &EPlayer) -> i32 {
    let opponents_choice = &round[0];
    let my_choice = &round[1];
    let outcome = get_outcome(opponents_choice, my_choice, player);
    match player {
        Opponent => choice_to_int(opponents_choice) + outcome_to_int(outcome),
        Me => choice_to_int(my_choice) + outcome_to_int(outcome),
    }
}

fn parse_round_str_part_1(roundstr: &str) -> Vec<Choice> {
    roundstr
        .split(" ")
        .map(|x| match x {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => {
                println!("erm, he's standing right behind me, isn't he?");
                ChoiceUnknown
            }
        })
        .collect()
}

fn get_my_choice_part_2(opponents_choice: &Choice, my_choice: &Choice) -> Choice {
    match opponents_choice {
        Rock => match my_choice {
            NeedWin => Paper,
            NeedDraw => Rock,
            NeedLose => Scissors,
            _ => {
                println!(
                    "this should literally never print. I am currently at get_my_choice_part_2"
                );
                ChoiceUnknown
            }
        },
        Paper => match my_choice {
            NeedWin => Scissors,
            NeedDraw => Paper,
            NeedLose => Rock,
            _ => {
                println!(
                    "this should literally never print. I am currently at get_my_choice_part_2"
                );
                ChoiceUnknown
            }
        },
        Scissors => match my_choice {
            NeedWin => Rock,
            NeedDraw => Scissors,
            NeedLose => Paper,
            _ => {
                println!(
                    "this should literally never print. I am currently at get_my_choice_part_2"
                );
                ChoiceUnknown
            }
        },
        _ => ChoiceUnknown,
    }
}

fn score_round_part_2(round: Vec<Choice>, _player: &EPlayer) -> i32 {
    let opponents_choice = &round[0];
    let my_choice = get_my_choice_part_2(opponents_choice, &round[1]);
    let outcome = get_outcome(opponents_choice, &my_choice, &Me);
    choice_to_int(&my_choice) + outcome_to_int(outcome)
}

fn parse_round_str_part_2(roundstr: &str) -> Vec<Choice> {
    roundstr
        .split(" ")
        .map(|x| match x {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            "X" => NeedLose,
            "Y" => NeedDraw,
            "Z" => NeedWin,
            _ => {
                println!("erm, he's standing right behind me, isn't he?");
                ChoiceUnknown
            }
        })
        .collect()
}

fn input_to_rounds(
    input: &str,
    round_scorer: &dyn Fn(Vec<Choice>, &EPlayer) -> i32,
    round_str_parser: &dyn Fn(&str) -> Vec<Choice>,
) -> Vec<i32> {
    input
        .trim()
        .split("\n")
        .map(|x| round_scorer(round_str_parser(x), &Me))
        .collect()
}

fn main() {
    let input = fs::read_to_string("input").expect("ERROR: unable to read input");

    println!("\n\nPART 1");
    let rounds_part_1 = input_to_rounds(&input, &score_round_part_1, &parse_round_str_part_1);
    println!("input to rounds is {:?}", rounds_part_1);
    println!("total score is {:?}", rounds_part_1.iter().sum::<i32>());

    println!("\n\nPART 2");
    let rounds_part_2 = input_to_rounds(&input, &score_round_part_2, &parse_round_str_part_2);
    println!("input to rounds is {:?}", rounds_part_2);
    println!("total score is {:?}", rounds_part_2.iter().sum::<i32>());
}
