/*
 * https://adventofcode.com/2022/day/3
 * - each rucksack has two large compartments, each item has a canonical compartment
 * - exactly 1 item per rucksack is in the wrong compartment
 * - first half of chars are in the first compartment, second half in second
 * - a-z have priorities 1-26, A-Z have priorities 27-52
 * - get every repeated item type, sum the priorities of these item types
 */

mod constants;

use colored::Colorize;
use constants::*;
use std::fs;

fn string_to_int(string: &str) -> Vec<&i32> {
    string
        .chars()
        .map(|x| match LETTER_MAP.get(&x) {
            Some(thing) => thing,
            None => {
                println!(
                    "{}: Hey, asshole, LETTER_MAP does not accept {:?}, you silly fool. You gave it: {:?}",
                    "WARNING".yellow(),
                    x,
                    string
                );
                &0 // 0 will not change the sum
            }
        })
        .collect()
}

fn find_repeated_item_in_backpack(backpack: Vec<&i32>) -> Option<&i32> {
    match backpack.len() {
        len if len % 2 == 0 && len != 0 => {
            let compartments: Vec<&[&i32]> = backpack.chunks(len / 2).collect();
            for item in compartments[0] {
                if compartments[1].iter().any(|&i| &i == item) {
                    return Some(&item);
                }
            }
            return None;
        }
        _ => None,
    }
}

fn find_repeated_item_in_three_backpacks(backpacks: Vec<Vec<&i32>>) -> Option<&i32> {
    match backpacks.len() {
        3 => {
            for item in 0..backpacks[0].len() {
                if backpacks[1].iter().any(|&i| i == backpacks[0][item]) {
                    if backpacks[2].iter().any(|&j| j == backpacks[0][item]) {
                        return Some(&backpacks[0][item]);
                    }
                }
            }
            return None;
        }
        _ => None,
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("ERROR: unable to read input");

    println!(
        "The sum of the priorities is: {}",
        input
            .trim()
            .split("\n")
            .map(|x| match find_repeated_item_in_backpack(string_to_int(x)) {
                Some(whatever) => whatever,
                None => {
                    println!("Yo, I got None for Some(reason)");
                    &0 // 0 will not change the sum
                }
            })
            .sum::<i32>()
    );

    println!(
        "Ummmm, uhhhhh, durrr: {}",
        input
            .trim()
            .split("\n")
            .collect::<Vec<&str>>()
            .chunks(3)
            .map(|x| find_repeated_item_in_three_backpacks(
                x.iter().map(|y| string_to_int(y)).collect()
            )
            .unwrap())
            .sum::<i32>()
    )
}
