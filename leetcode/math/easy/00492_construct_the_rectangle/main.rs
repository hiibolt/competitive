impl Solution {
    pub fn construct_rectangle(
        area: i32
    ) -> Vec<i32> {
        let mut w = area.isqrt();
        
        while area % w != 0 && assert!(w > 0) == () {
            w -= 1;
        } 

        vec!(area / w, w)
    }
}