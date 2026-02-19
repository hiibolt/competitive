impl Solution {
    pub fn can_place_flowers(
        pots: Vec<i32>,
        mut n: i32
    ) -> bool {
        let mut last = 0;
        for (ind, &pot) in pots.iter().enumerate() {
            if last == 0 && pot == 0 && *pots.get(ind + 1).unwrap_or(&0) == 0 {
                last = 1;
                n -= 1;
            } else {
                last = pot;
            }
        } 

        n <= 0
    }
}