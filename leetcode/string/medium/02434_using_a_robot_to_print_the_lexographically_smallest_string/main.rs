/**
 * 2434. "Using a Robot to Print the Lexicographically Smallest String"
 * 
 * Difficulty: Medium
 * Tags: Hash Table, String, Stack, Greedy
 * Runtime: Beats 9% (T~T)
 */
use std::collections::VecDeque;
impl Solution {
    pub fn robot_with_string ( mut s: String ) -> String {
        let mut ret = String::new();
        let mut t = VecDeque::new();

        'outer: while !s.is_empty() {
            for ch in 'a'..='z' {
                while let Some(last) = t.iter().last() {
                    if last == &ch {
                        ret.push(t.pop_back().unwrap());
                        continue 'outer;
                    }
                    break;
                }
                while s.contains(ch) {
                    let removed = s.remove(0);
                    t.push_back(removed);

                    if removed == ch {
                        while let Some(last) = t.iter().last() {
                            if last == &ch {
                                ret.push(t.pop_back().unwrap());
                                continue 'outer;
                            }
                            break;
                        }
                    }
                }
            }
        }

        while let Some(removed) = t.pop_back() {
            ret.push(removed);
        }

        ret
    }
}