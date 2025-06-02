use std::collections::HashMap;

fn main() {
    let contents: String = std::fs::read_to_string("input.txt")
        .unwrap();

    // Extract our dataset
    let mut left = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .next().unwrap()
                .parse::<u64>().unwrap()
        })
        .collect::<Vec<u64>>();

    let mut right = HashMap::new();
    for line in contents.lines() {
        let num = 
        line.split_whitespace()
            .skip(1).next().unwrap()
            .parse::<u64>().unwrap();

        *(right.entry(num).or_insert(0)) += 1u64;
    }
    left.sort();

    println!("contents: {:#?}\n\n{:#?}", left, right);

    // Compute the final result
    let mut total_similarity = 0u64;
    for i in 0..left.len() {
        total_similarity += left[i] * right.get(&left[i]).unwrap_or(&0u64);
    }

    println!("Result: {}", total_similarity);
}
