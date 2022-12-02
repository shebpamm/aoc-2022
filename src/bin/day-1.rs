use aoc::puzzle;
use std::io::prelude::*;

fn main() {
    let input = puzzle::get_puzzle_input(1).unwrap();

    let mut highest = vec![0; 3];

    let mut current_bag = 0;

    for line in input.lines() {
        let line = line.unwrap(); // Shadow with unwrap

        match line.as_str() {
            "" => {
                for score in highest.iter_mut() {
                    if current_bag > *score {
                        *score = current_bag;
                        break;
                    }
                }

                current_bag = 0;
                continue;
            }
            value => current_bag = current_bag + value.parse::<i32>().unwrap(),
        }
    }

    println!("{:?}", highest);
}
