impl Solution {
    pub fn image_smoother(
        mut img: Vec<Vec<i32>>
    ) -> Vec<Vec<i32>> {
        let mut averages = vec!(vec!(0; img[0].len()); img.len());

        for i in 0..img.len() {
            for g in 0..img[i].len() {
                let mut total = 0i32;
                let mut count = 0i32;
                for inner_i in (i.saturating_sub(1).max(0))..=((i + 1).min(img.len() - 1)) {
                    for inner_g in (g.saturating_sub(1).max(0))..=((g + 1).min(img[0].len() - 1)) {
                        total += img[inner_i][inner_g];
                        count += 1;
                    }
                }
                averages[i][g] = total / count;
            }
        }

        averages
    }
}