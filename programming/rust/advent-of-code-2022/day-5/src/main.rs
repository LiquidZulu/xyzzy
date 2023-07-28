/*
 * https://adventofcode.com/2022/day/5
 */
// uncomment this when writing the thing, comment it when you actually want the thing done
//#![allow(unused)]
mod constants;

use colored::Colorize;
use constants::*;
use std::collections::VecDeque;
use std::fs;

type Stack = VecDeque<char>;
type Instruction = Vec<i32>;

fn char_to_int(c: &char) -> &i32 {
    match LETTER_MAP.get(&c) {
        Some(thing) => thing,
        None => &0, // I use 0 to indicate that it is not a valid letter; 0 wont crash the program but also wont be detected as something to add to the stacks
    }
}

//  1   5   9   13  17  21  25  29  33
// [W] [B] [H] [F] [L] [F] [J] [V] [B]
// starts on 1 and increments by 4 each time
fn get_stack_positions(n_stacks: i32) -> Vec<i32> {
    let mut offset = -3;
    (0..n_stacks)
        .map(|_| {
            offset += 4;
            offset
        })
        .collect::<Vec<i32>>()
}

fn get_stacks(stacks_str: &str, n_stacks: i32) -> Vec<Stack> {
    let mut stacks = (0..n_stacks)
        .map(|_| VecDeque::from([]))
        .collect::<Vec<Stack>>();
    let stack_positions = get_stack_positions(n_stacks);

    for row in stacks_str.split('\n') {
        for column_n in 0..n_stacks {
            let the_character_im_on = row.chars().nth(
                TryInto::<usize>::try_into(
                    stack_positions[TryInto::<usize>::try_into(column_n).unwrap()],
                )
                .unwrap(),
            );
            match the_character_im_on {
                Some(_) => match char_to_int(&the_character_im_on.unwrap()) {
                    1..=26 => {
                        stacks[TryInto::<usize>::try_into(column_n).unwrap()]
                            .push_back(the_character_im_on.unwrap());
                    }
                    _ => (),
                },
                _ => (),
            };
        }
    }

    stacks
}

fn get_instructions(instructions_str: &str) -> Vec<Instruction> {
    instructions_str
        .trim()
        .split('\n')
        .map(|instruction| {
            instruction
                .split(' ')
                .filter(|x| match x {
                    &"move" | &"from" | &"to" => false,
                    _ => true,
                })
                .map(|x| match x.parse() {
                    Ok(thing) => thing,
                    Err(e) => {
                        println!("{}: {} for {:?}", "Ruh roh raggy".red(), e, x);
                        0
                    }
                })
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

// -> stacks, instructions
fn parse_input(input: String, n_stacks: i32) -> (Vec<Stack>, Vec<Instruction>) {
    let split_input = input.split("\n\n").collect::<Vec<&str>>();

    (
        get_stacks(split_input[0], n_stacks),
        get_instructions(split_input[1]),
    )
}

fn apply_instructions_part_1(stacks: &mut Vec<Stack>, instructions: Vec<Instruction>) -> () {
    for i in instructions.iter() {
        assert_eq!(i.len(), 3);
        let (n_to_move, from_stack, to_stack) = (i[0], i[1] - 1, i[2] - 1);
        for _ in 0..n_to_move {
            let popped = stacks[TryInto::<usize>::try_into(from_stack).unwrap()]
                .pop_front()
                .unwrap();
            stacks[TryInto::<usize>::try_into(to_stack).unwrap()].insert(0, popped)
        }
    }
}

fn apply_instructions_part_2(stacks: &mut Vec<Stack>, instructions: Vec<Instruction>) -> () {
    for i in instructions.iter() {
        assert_eq!(i.len(), 3);
        let (n_to_move, from_stack, to_stack) = (i[0], i[1] - 1, i[2] - 1);
        let mut popped: Stack = stacks[TryInto::<usize>::try_into(from_stack).unwrap()]
            .drain(0..TryInto::<usize>::try_into(n_to_move).unwrap())
            .collect();
        popped.append(&mut stacks[TryInto::<usize>::try_into(to_stack).unwrap()]);
        stacks[TryInto::<usize>::try_into(to_stack).unwrap()] = popped;
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("ERROR: unable to read input");
    let (mut stacks1, instructions) = parse_input(input, 9);
    let mut stacks2 = stacks1.clone();

    apply_instructions_part_1(&mut stacks1, instructions.clone());
    apply_instructions_part_2(&mut stacks2, instructions);
    println!(
        "{:?}",
        stacks1
            .iter()
            .map(|stack| stack[0])
            .collect::<VecDeque<char>>()
    );
    println!(
        "{:?}",
        stacks2
            .iter()
            .map(|stack| stack[0])
            .collect::<VecDeque<char>>()
    );
}
