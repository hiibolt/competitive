/**
 * 43. "Multiply Strings"
 * 
 * Difficulty: Medium
 * Tags: Math, String, Simulation
 * Runtime: Beats 20%
 */
impl Solution {
    pub fn multiply(
        mut num1: String,
        mut num2: String
    ) -> String {
        if num2.len() > num1.len() {
            std::mem::swap(&mut num1, &mut num2);
        }
        num1 = num1.chars().rev().collect::<String>();
        num2 = num2.chars().rev().collect::<String>();

        // Create our computation table
        let mut comp_table = vec!(vec!(0; num1.len() + num2.len()); num2.len());

        for (ind_2, ch_2) in num2.chars().enumerate() {
            let digit_2 = ch_2.to_digit(10).unwrap();
            let mut carry = 0;

            for (ind_1, ch_1) in num1.chars().enumerate() {
                let digit_1 = ch_1.to_digit(10).unwrap();
                let new_total = digit_2 * digit_1 + carry; 

                if ( new_total > 9 ) {
                    comp_table[ind_2][ind_1 + ind_2] = new_total % 10;
                    carry = new_total / 10;
                } else {
                    comp_table[ind_2][ind_1 + ind_2] = new_total;
                    carry = 0;
                }
            }

            comp_table[ind_2][num1.len() + ind_2] = carry;
        }

        println!("Computation table: {comp_table:?}");

        let mut carry = 0;
        let mut ret = String::new();
        for col in 0..comp_table[0].len() {
            let mut total = carry;
            for row in 0..comp_table.len() {
                total += comp_table[row][col];
            }
            if total > 9 {
                ret += &format!("{}", total % 10);
                carry = total / 10;
            } else {
                ret += &format!("{}", total);
                carry = 0;
            }
        }

        let ret = ret.chars().rev().skip_while(|e| e == &'0').collect::<String>();
        if ret.len() == 0 {
            String::from("0")
        } else {
            ret
        }
    }
}