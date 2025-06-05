/**
 * 380. "Insert Delete GetRandom O(1)"
 * 
 * Difficulty: Medium
 * Tags: Array, Hash Table, Math, Design, Randomized
 * Runtime: Beats 44%
 */
use std::collections::HashSet;
use rand::prelude::*;
struct RandomizedSet {
    len: f32,
    state: HashSet<i32>
}
impl RandomizedSet {
    fn new() -> Self {
        Self {
            len: 0f32,
            state: HashSet::new()
        }
    }
    fn insert(&mut self, val: i32) -> bool {
        if self.state.insert(val) {
            self.len += 1f32;
            true
        } else {
            false
        }
    }
    fn remove(&mut self, val: i32) -> bool {
        if self.state.remove(&val) {
            self.len -= 1f32; 
            true
        } else {
            false
        }
    }
    fn get_random(&self) -> i32 {
        *self.state
            .iter()
            .nth((random::<f32>() * self.len) as usize)
            .unwrap()
    }
}