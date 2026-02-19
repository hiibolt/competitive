use std::collections::HashMap;
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let elements = nums.into_iter()
            .fold(HashMap::new(), |mut hm, val| {
                *(hm.entry(val).or_insert(0i32)) += 1;
                hm
            });

        let mut ret = 0i32;
        for (&key, val_1) in elements.iter() {
            if let Some(&val_2) = elements.get(&(key + 1)) {
                ret = ret.max(*val_1 + val_2);
            }
        }
        ret
    }
}