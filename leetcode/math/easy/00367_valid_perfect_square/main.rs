/**
 * 367. "Valid Perfect Square"
 * 
 * Difficulty: Easy
 * Tags: Math, Binary Search
 */
impl Solution {
    fn perfect_square_recurse (
        lower: i32,
        upper: i32,
        target: i32
    ) -> bool {
        println!("{lower}-{upper} ({target})");
        if upper - lower < 0 {
            return false;
        }
        let avg = ( lower + upper ) / 2;
        let avg_squared: i128 = avg as i128 * avg as i128;

        if avg_squared > target as i128 {
            return Self::perfect_square_recurse(lower, avg - 1, target);
        } else if avg_squared < target as i128 {
            return Self::perfect_square_recurse(avg + 1, upper, target);
        }

        true
    }
    pub fn is_perfect_square (
        num: i32
    ) -> bool {
        Self::perfect_square_recurse(0, num, num)
    }
}