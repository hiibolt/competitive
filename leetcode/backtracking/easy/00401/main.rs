/**
 * 401. "Binary Watch"
 * 
 * Difficulty: Easy
 * Tags: Junior, Backtracking, Bit Manipulation
 * Runtime: Beats 100%
 */
const VALUES: [(u8, u8); 10] = [
    (8, 0), (4, 0), (2, 0), (1, 0),
    (0, 32), (0, 16), (0, 8), (0, 4), (0, 2), (0, 1)
];

impl Solution {
    pub fn read_binary_watch(
        turned_on: i32
    ) -> Vec<String> {
        if turned_on > 8 { return vec!(); }

        Self::permute_time((0, 0), 0, turned_on)
            .into_iter()
            .map(|(hours, minutes)| {
                format!("{hours}:{minutes:02}")
            })
            .collect()
    }
    fn permute_time (
        time: (u8, u8),
        index: usize,
        leds: i32
    ) -> Vec<(u8, u8)> {
        if leds == 0 && time.0 < 12 && time.1 < 60 {
            return vec!(time);
        }
        if index >= 10 || time.0 >= 12 || time.1 >= 60 {
            return vec!();
        }
        println!("{index} {:?}", VALUES[index]);

        let mut ret = Vec::new();
        ret.append(&mut Self::permute_time(
            time,
            index + 1,
            leds
        ));
        ret.append(&mut Self::permute_time(
            (time.0 + VALUES[index].0, time.1 + VALUES[index].1),
            index + 1,
            leds - 1
        ));

        return ret;
    }
}