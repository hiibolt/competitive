/**
 * 3307. "Find the Kth Character in String Game II"
 * 
 * Difficulty: Hard
 * Tags: Math, Bit Manipulation, Recursion
 * Runtime: Beats 100%
 * 
 * Notes:
 *  Very effective solution using math. Always neat
 *   to analyze the problem and look for patterns -
 *   you can find the pattern I found at the bottom
 *   of the file ^^
 * 
 *  A good example of why looking at the problem input
 *   can tell you the expected runtime complexity. In
 *   this case, it was massive (10^14) and so a
 *   brute force (simulation) solution would not work - 
 *   instead, we look for an O(n) solution.
 */
use std::collections::LinkedList;
impl Solution {
    pub fn kth_character(
        mut k: i64,
        operations: Vec<i32>
    ) -> char {
        k = if k == 1 { return 'a' } else { k - 1 };
        let mut effective_indexes = LinkedList::new();

        let mut steps = k.ilog2();
        for step in (0..=steps).rev() {
            effective_indexes.push_front(k);
            k = k % 2i64.pow(step);
        }
        
        let mut current_char: u8 = 0;
        for (ind, effective_index) in effective_indexes.into_iter().enumerate() {
            let string_len = 2i64.pow(ind as u32 + 1);
            let would_affect = effective_index >= (string_len / 2);

            if would_affect && operations[ind] == 1 {
                current_char += 1;
                current_char = current_char % 26;
            }
        }

        (current_char + 97) as char
    }
}
/**
aabbbbcc bbccccdd - 1 (11 % 2^3 = 3)
aabbbbcc          - 1 (3  % 2^2 = 3)
aabb              - 1 (3  % 2^1 = 1)
aa                - 0 (1  % 2^0 = 0)
a

aabbaabb bbccbbcc - 1 - (10 % 2^3 = 2)
aabbaabb          - 0 - (2  % 2^2 = 2)
aabb              - 1 - (2  % 2^1 = 0)
aa                - 0 - (0  % 2^0 = 0)
a
*/