/*
 * 3443. "Maximum Manhattan Distance After K Changes"
 * 
 * Difficulty: Medium
 * Tags: Hash Table, Math, String, Counting
 * Runtime: Beats 8%
 */
use std::collections::HashMap;
impl Solution {
    pub fn max_distance ( 
        s: String,
        k: i32
    ) -> i32 {
        let no_changes = s.chars()
            .fold((0i32, 0i32), |acc, ch| {
                let next_move = match ch {
                    'N' => (0, 1),
                    'E' => (1, 0),
                    'S' => (0, -1),
                    _ => (-1, 0)
                };
                (acc.0 + next_move.0, acc.1 + next_move.1)
            });
        let mut max = no_changes.0.abs() + no_changes.1.abs();

        for going_up in 0..=1 {
            for going_right in 0..=1 {
                let mut coords = (0i32, 0i32);
                let mut k = k;
                for ch in s.chars() {
                    let next_move = match ch {
                        'N' => {
                            if going_up == 0 && k > 0 {
                                k -= 1;
                                (0, -1)
                            } else {
                                (0, 1)
                            }
                        },
                        'E' => {
                            if going_right == 0 && k > 0 {
                                k -= 1;
                                (-1, 0)
                            } else {
                                (1, 0)
                            }
                        },
                        'S' => {
                            if going_up == 1 && k > 0 {
                                k -= 1;
                                (0, 1)
                            } else {
                                (0, -1)
                            }
                        },
                        _ => {
                            if going_right == 1 && k > 0 {
                                k -= 1;
                                (1, 0)
                            } else {
                                (-1, 0)
                            }
                        }
                    };
                    coords.0 += next_move.0;
                    coords.1 += next_move.1;
                    max = max.max(coords.0.abs() + coords.1.abs());
                }
            }
        }


        max
    }
}