use std::sync::{ Arc, Mutex };
use memoize::memoize;

#[memoize]
fn blink_stone (
    blinks: u32,
    current_blinks: u32,
    stone: u128
) -> u128 {
    if blinks == current_blinks {
        return 1;
    }

    if stone == 0 { 
        return blink_stone(blinks, current_blinks + 1, 1);
    }

    let stone_str = stone.to_string();
    let stone_str_len = stone_str.len();
    if stone_str_len % 2 == 0 {
        let left = stone_str.chars()
            .take(stone_str_len / 2)
            .collect::<String>()
            .parse::<u128>()
            .unwrap();
        let right = stone_str.chars()
            .skip(stone_str_len / 2)
            .take(stone_str_len / 2)
            .collect::<String>()
            .parse::<u128>()
            .unwrap();

        return blink_stone(blinks, current_blinks + 1, left) +
                blink_stone(blinks, current_blinks + 1, right);
    }

    return blink_stone(blinks, current_blinks + 1, stone * 2024);
}
fn main() {
    let stones = std::fs::read_to_string("input.txt")
        .unwrap()
        .split_ascii_whitespace()
        .map(|st| st.parse::<u128>().unwrap() )
        .collect::<Vec<u128>>();

    let mut handles = Vec::new();
    let total = Arc::new(Mutex::new(0u128));
    for &stone in &stones {
        let total_handle = total.clone();
        handles.push(std::thread::spawn( move || {
            let inner_total = blink_stone(75, 0, stone);

            println!("Finished a stone");

            // Add it to the global total
            *(total_handle.lock().expect("Failed to lock global total!")) += inner_total;
        }));
    }

    for handle in handles {
        handle.join().expect("Thread join failed!");
    }

    let final_total = *(total.lock().expect("Failed to lock final value!"));
    println!("There are {} stones", final_total);
}
