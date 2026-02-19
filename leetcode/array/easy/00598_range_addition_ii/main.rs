impl Solution {
    pub fn max_count( m: i32, n: i32, ops: Vec<Vec<i32>> ) -> i32 {
        let (width, height) = ops
            .into_iter()
            .fold((m, n), |(height, width), op| (height.min(op[0]), width.min(op[1])));
        
        width * height
    }
}