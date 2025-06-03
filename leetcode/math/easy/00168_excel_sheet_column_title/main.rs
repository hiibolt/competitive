/**
 * 168. "Excel Sheet Column Title"
 * 
 * Difficulty: Easy
 * Tags: Math, String
 * Runtime: 100%
 */
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut ret = String::new();

        let mut num: u32 = column_number as u32;
        while ( num > 0 ) {
            let m: u32 = num % 26;
            if (m == 0) {
                num -= 1;
                ret.insert(0, 'Z');
            } else {
                ret.insert(0, char::from_u32(64 + m).unwrap());
            }

            num -= m;
            num /= 26;
        }

        ret
    }
}