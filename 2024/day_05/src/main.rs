use std::collections::HashMap;

fn is_valid_update (
    update: HashMap<&str, usize>,
    instruction_str: &str
) -> bool {
    for instruction in instruction_str.lines() {
        let first = instruction.split('|').next().unwrap();
        let second = instruction.split('|').skip(1).next().unwrap();

        match (update.get(first), update.get(second)) {
            (Some(f_ind), Some(s_ind)) => {
                if f_ind > s_ind {
                    return false;
                }
            },
            _ => {}
        }
    }

    return true;
}
fn as_hm<'a> ( vc: &Vec<&'a str> ) -> HashMap<&'a str, usize> {
    let mut update: HashMap<&'a str, usize> = HashMap::new();
    let mut index = 0usize;
    for page in vc {
        update.insert(page, index);
        index += 1;
    }

    update
}
fn main() {
    let data = std::fs::read_to_string("input.txt")
        .unwrap();
    let instruction_str = data.split("\n\n").next().unwrap();
    let pages_str = data.split("\n\n").skip(1).next().unwrap();

    let mut sum = 0i32;
    for (ind, line) in pages_str.lines().enumerate() {
        let mut pages: Vec<&str> = line.split(',').collect();
        
        if is_valid_update(as_hm(&pages), instruction_str) {
            continue;
            //sum += pages[pages.len() / 2].parse::<i32>().unwrap();
        }
        println!("Update #{ind}");
        let mut reconstructed = vec!(pages[0]);
        println!("{pages:?}");

        for i in 1..pages.len() {
            let new_element = pages[i];
            println!("{reconstructed:?}");

            let mut left_cursor = 0;
            let mut right_cursor = reconstructed.len();
            for instruction in instruction_str.lines() {
                let first = instruction.split('|').next().unwrap();
                let second = instruction.split('|').skip(1).next().unwrap();

                
                if first == new_element && reconstructed.contains(&second) {
                    //println!("GE instruction: {first}|{second} ({left_cursor}-{right_cursor})");
                    right_cursor = right_cursor.min(
                        reconstructed.iter()
                            .position(|&st| st == second)
                            .unwrap() + 1
                    );
                    // println!("Now ({left_cursor}-{right_cursor})");
                }
                if second == new_element && reconstructed.contains(&first) {
                    //println!("LT instruction: {first}|{second} ({left_cursor}-{right_cursor})");
                    left_cursor = left_cursor.max(
                        reconstructed.iter()
                            .position(|&st| st == first)
                            .unwrap() + 1
                    );
                    //println!("Now ({left_cursor}-{right_cursor})");
                }

            }

            reconstructed.insert(left_cursor, new_element);
        }
        
        sum += reconstructed[reconstructed.len() / 2].parse::<i32>().unwrap();
    }

    println!("Sum: {sum}");
}
