/**
 * 415. "Add Strings"
 * 
 * Difficulty: Easy
 * Tags: Math, String, Simulation
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn add_strings(mut num1: String, num2: String) -> String {
        let arr_1: Vec<u32> = num1.chars()
            .rev()
            .map(|ch| (ch as u32 - 48) )
            .collect::<Vec<u32>>();
        let arr_2: Vec<u32> = num2.chars()
            .rev()
            .map(|ch| (ch as u32 - 48) )
            .collect::<Vec<u32>>();
        
        let mut combined: Vec<u32> = vec!(0; arr_1.len().max(arr_2.len()) );

        for i in 0..combined.len() {
            combined[i] = arr_1.get(i).unwrap_or(&0) + arr_2.get(i).unwrap_or(&0);
        }

        drop(arr_1);
        drop(arr_2);

        while combined.iter().any(|&item| item >= 10) {
            for i in 0..combined.len() {
                if combined[i] >= 10 {
                    combined[i] = combined[i] - 10;

                    if let Some(mut num) = combined.get_mut(i + 1) {
                        *num += 1;
                    } else {
                        combined.push(1);
                    }
                }
            }
        }

        combined.into_iter()
            .rev()
            .flat_map(|num| char::from_u32(num + 48).and_then(|val| Some(String::from(val))))
            .collect::<Vec<String>>()
            .join("")
    }
}