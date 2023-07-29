/*
 * https://adventofcode.com/2022/day/6
 */

use std::collections::HashSet;
use std::fs;
use std::hash::Hash;

// This seems fine, I can't tell if it is already in std or not: https://stackoverflow.com/a/46767732
fn unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn get_unique_run(datastream: &Vec<char>, run_len: i32) -> Option<i32> {
    let mut char_index = run_len;
    match run_len.try_into() {
        Ok(thing) => {
            for window in datastream.windows(thing) {
                if unique_elements(window) {
                    return Some(char_index);
                }
                char_index += 1;
            }
        }
        Err(_) => return None,
    }
    None
}

fn main() {
    let input = fs::read_to_string("input").expect("ERROR: unable to read input");
    let datastream = input.chars().collect();

    match get_unique_run(&datastream, 4) {
        Some(thing) => println!("First 4-run after character {thing}"),
        None => println!("Marker could not be found in the input."),
    }

    match get_unique_run(&datastream, 14) {
        Some(thing) => println!("First 14-run after character {thing}"),
        None => println!("Marker could not be found in the input."),
    }
}
