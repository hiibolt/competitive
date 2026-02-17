use std::collections::BinaryHeap;
impl Solution {
    pub fn find_relative_ranks(
        mut scores: Vec<i32>
    ) -> Vec<String> {
        let mut heap = BinaryHeap::new();
        for (ind, &score) in scores.iter().enumerate() {
            heap.push((score, ind));
        }

        let mut ret: Vec<String> = vec!(String::new(); scores.len());
        let mut place = 1;
        while let Some((_, ind)) = heap.pop() {
            ret[ind] = match place {
                1 => "Gold Medal".into(),
                2 => "Silver Medal".into(),
                3 => "Bronze Medal".into(),
                other => other.to_string()
            };
            place += 1;
        }

        ret
    }
}