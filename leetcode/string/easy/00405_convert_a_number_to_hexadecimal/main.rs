const DIGITS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'
];

impl Solution {
    pub fn to_hex(
        num: i32
    ) -> String {
        let mut num = if num < 0 { 
            u32::MAX + num as u32 + 1
        } else if num == 0 {
            return String::from("0");
        } else { 
            num as u32
        };

        let mut ret = String::new();
        while num > 0 {
            ret.insert(0, DIGITS[(num % 16) as usize]);
            num /= 16;
        }
        
        ret
    }
}