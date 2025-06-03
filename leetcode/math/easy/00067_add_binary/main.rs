/**
 * 67. "Add Binary"
 * 
 * Difficulty: Easy
 * Tags: Math, String, Bit Manipulation, Simulation
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn add_binary(_a: String, _b: String) -> String {
        let mut a: Vec<char> = (if _a.len() > _b.len() { _a.clone() } else { _b.clone( ) }).chars().collect();
        let mut b: Vec<char> = (if _a.len() > _b.len() { _b } else { _a }).chars().collect();

        let mut ind = 0;
        while ind < b.len() {
            let a_len = a.len();
            if b[b.len() - ind - 1] == '0' {
                ind += 1;
                continue;
            }
            if a[a_len - ind - 1] == '0' {
                a[a_len - ind - 1] = '1';
                ind += 1;
                continue;
            }

            let mut ind_offset = (a_len - ind - 1) as i32;
            let mut edge = false;
            'inner: while a[ind_offset as usize] == '1' {
                a[ind_offset as usize] = '0';
                ind_offset -= 1;
                if ind_offset < 0 {
                    a.insert(0, '1');
                    edge = true;
                    break 'inner;
                }
            }
            if !edge {
                a[ind_offset as usize] = '1';
            }
            ind += 1;
        }
        a.iter().collect::<String>()
    }
}