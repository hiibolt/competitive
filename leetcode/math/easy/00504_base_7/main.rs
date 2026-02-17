impl Solution {
    pub fn convert_to_base7(
        mut num: i32
    ) -> String {
        let mut plus = if num < 0 { num = -num; "-" } else if num == 0 { return "0".into(); } else { "" };
        
        let mut ret = String::new();
        while num > 0 {
            ret.insert(0, (num % 7 + 48) as u8 as char);
            num /= 7;
        }

        format!("{plus}{ret}")
    }
}