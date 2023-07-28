/*
 * https://adventofcode.com/2022/day/4
 */

mod get_intersection;

use colored::Colorize;
use get_intersection::*;
use std::fs;

fn parse_task_range(task_range: &str) -> Option<(i32, i32)> {
    let range: Vec<i32> = task_range
        .split("-")
        .map(|x| match x.parse() {
            Ok(n) => n,
            Err(e) => {
                println!("{}: parse_task_range has errored: {:?}\n\t ↳ This probably means that you have not given it something in the format \"x-y\" where \"x\" and \"y\" are numbers.", "ERROR".red(), e);
                -1
            }
        })
        .collect();

    match range.len() {
        2 => match (range[0], range[1]) {
            (a, b) if a > b => Some((b, a)),
            (a, b) if a < b => Some((a, b)),
            (a, b) => Some((a, b)),
        },
        _ => None,
    }
}

fn task_pair_str_to_tuple(task_pair: &str) -> Option<((i32, i32), (i32, i32))> {
    let pair_vec: Vec<(i32, i32)> = task_pair
        .split(",")
        .map(|x| match parse_task_range(&x) {
            Some((a, b)) => (a, b),
            _ => (0, 0),
        })
        .collect();

    match pair_vec.len() {
        2 => Some((pair_vec[0], pair_vec[1])),
        _ => None,
    }
}

fn fully_contains(pair: ((i32, i32), (i32, i32))) -> bool {
    match get_intersection(pair) {
        Some(o) => match (pair, o) {
            ((a, b), o) if a == o || b == o => true,
            _ => false,
        },
        None => false,
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("ERROR: unable to read input");

    tests();

    println!(
        "{:?} pairs have one fully contain the other.",
        input
            .trim()
            .split("\n")
            .map(|x| if fully_contains(task_pair_str_to_tuple(x).unwrap()) {
                //                println!("{x}\n - True");
                1
            } else {
                //              println!("{x}\n - False");
                0
            })
            .sum::<i32>()
    );

    println!(
        "{:?} pairs overlap.",
        input
            .trim()
            .split("\n")
            .map(
                |x| match get_intersection(task_pair_str_to_tuple(x).unwrap()) {
                    Some(_) => 1,
                    None => 0,
                }
            )
            .sum::<i32>()
    )
}

fn tests() {
    assert_eq!(
        fully_contains(task_pair_str_to_tuple("1-2,1-5").unwrap()),
        true
    );
    assert_eq!(
        fully_contains(task_pair_str_to_tuple("2-3,1-5").unwrap()),
        true
    );
    assert_eq!(
        fully_contains(task_pair_str_to_tuple("4-5,1-5").unwrap()),
        true
    );
    assert_eq!(
        fully_contains(task_pair_str_to_tuple("1-5,1-2").unwrap()),
        true
    );
    assert_eq!(
        fully_contains(task_pair_str_to_tuple("1-5,2-3").unwrap()),
        true
    );
    assert_eq!(
        fully_contains(task_pair_str_to_tuple("1-5,4-5").unwrap()),
        true
    );
    assert_eq!(
        fully_contains(task_pair_str_to_tuple("1-3,5-10").unwrap()),
        false
    );
    assert_eq!(
        fully_contains(task_pair_str_to_tuple("3-5,5-10").unwrap()),
        false
    );
    assert_eq!(
        fully_contains(task_pair_str_to_tuple("3-7,5-10").unwrap()),
        false
    );
    assert_eq!(
        fully_contains(task_pair_str_to_tuple("8-11,5-10").unwrap()),
        false
    );
    assert_eq!(
        fully_contains(task_pair_str_to_tuple("10-12,5-10").unwrap()),
        false
    );
}
