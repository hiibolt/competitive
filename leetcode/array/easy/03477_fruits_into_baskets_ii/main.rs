/**
 * 3477. "Fruits into Baskets II"
 * 
 * Difficulty: Easy
 * Tags: Array, Binary Search, Segment Tree, Simulation, Ordered Set, Weekly Contest 440
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn num_of_unplaced_fruits(
        fruits: Vec<i32>,
        mut baskets: Vec<i32>
    ) -> i32 {
        let mut ret = 0i32;

        'outer: for fruit in fruits {
            let mut ind = 0;
            while ind < baskets.len() {
                if baskets[ind] >= fruit {
                    baskets.remove(ind);
                    continue 'outer;
                }
                ind += 1;
            }
            ret += 1;
        }

        ret
    }
}