/*
 * https://adventofcode.com/2022/day/1
 * - each Elf is carrying a certain number of calories
 * - list of calories, find elf carrying the most calories and how many calories that is
 */

use std::convert::TryFrom;
use std::fs;

fn parse_calories(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|c| c.lines().filter_map(|l| l.parse::<u32>().ok()).sum())
        .collect()
}

fn find_the_elf(input: &Vec<u32>) -> usize {
    let mut max: u32 = 0;
    let mut max_i: usize = 0;
    for i in 0..input.len() {
        if input[i] > max {
            max = input[i];
            max_i = i;
        }
    }
    max_i
}

fn top_n_elves(elves: &mut Vec<u32>, n: u32) -> Vec<u32> {
    let number_of_elves: u32 = u32::try_from(elves.len()).unwrap();

    elves.sort();
    let mut the_top_elves = vec![0; n.try_into().unwrap()];

    for i in 0..n {
        the_top_elves[usize::try_from(i).unwrap()] =
            elves[usize::try_from(number_of_elves - i - 1).unwrap()];
    }

    the_top_elves
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("ERROR: unable to read input.txt");
    let calories: Vec<u32> = parse_calories(&contents);
    let elf: usize = find_the_elf(&calories);
    println!("Elf {elf} has the most calories:");
    println!("{}\n\n", calories[elf]);

    let mut elves = calories;
    let top_elves = top_n_elves(&mut elves, 3);
    println!("The top three elves are: {:?}", top_elves);

    println!("Their sum is: {}", top_elves.iter().sum::<u32>())
}
