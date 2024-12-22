use std::collections::{HashMap, HashSet};

use memoize::memoize;

const ITERATIONS: usize = 2000;

#[memoize]
fn next ( secret: usize ) -> usize {
    let step_1 = step_1(secret);
    let step_2 = step_2(step_1);
    step_3(step_2)
}
#[memoize]
fn step_1 ( secret: usize ) -> usize {
    prune(mix(secret * 64, secret))
}
#[memoize]
fn step_2 ( secret: usize ) -> usize {
    prune(mix(secret / 32, secret))
}
#[memoize]
fn step_3 ( secret: usize ) -> usize {
    prune(mix(secret * 2048, secret))
}
#[memoize]
fn mix ( given: usize, secret: usize ) -> usize {
    given ^ secret
}
#[memoize]
fn prune ( secret: usize ) -> usize {
    secret % 16777216
}
fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<usize>().unwrap() )
        .collect::<Vec<usize>>();

    let mut sum = 0usize;
    let mut changes: (i32, i32, i32, i32) = (i32::MIN, i32::MIN, i32::MIN, i32::MIN);
    let mut all_change_values = Vec::new();
    for mut secret in input {
        println!("Secret `{secret}`");

        let mut last_digit = secret % 10;
        let mut change_values: HashMap<(i32, i32, i32, i32), usize> = HashMap::new();
    
        for i in 0..ITERATIONS {
            secret = next(secret);

            let digit = secret % 10;
            let diff: i32 = digit as i32 - last_digit as i32;

            // Adjust the changes
            changes.0 = changes.1;
            changes.1 = changes.2;
            changes.2 = changes.3;
            changes.3 = diff;

            if i > 2 {
                if !change_values.contains_key(&changes) {
                    change_values.insert(changes, digit);
                } else {
                    //println!("occupied :3");
                }
            }

            //println!("{digit}: {diff}");
            last_digit = digit;
        }
    
        //println!("After {ITERATIONS} iterations: {secret}");
        //println!("Change values: {change_values:?}");
        all_change_values.push(change_values);

        sum += secret;
    }
    println!("Total sum: {sum}");

    let all_keys = all_change_values.iter()
        .flat_map(|hm| hm.keys())
        .collect::<HashSet<&(i32, i32, i32, i32)>>();

    let mut most_bananas = 0usize;
    for key in all_keys.into_iter() {
        let mut current_bananas = 0usize;

        for hm in all_change_values.iter() {
            current_bananas += hm.get(key).unwrap_or(&0usize);
        }

        most_bananas = most_bananas.max(current_bananas);
        println!("{key:?} - {current_bananas}");
    }
    println!("Highest possible banana count: {most_bananas}");
}
