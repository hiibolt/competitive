fn check ( 
    nums: &Vec<i32> 
) -> bool {
    let mut increasing: Option<bool> = None;
    let mut last: Option<i32> = None;
    for &num  in nums {
        if let Some(last) = last {
            if let Some(increasing) = increasing {
                if increasing && num < last {
                    return false;
                }
                if !increasing && num > last {
                    return false;
                }

                let diff: i32 = (num - last).abs();
                if diff < 1 || diff > 3 {
                    return false;
                }
            } else {
                let diff: i32 = (num - last).abs();
                if diff < 1 || diff > 3 {
                    return false;
                }
                increasing = Some(num > last);
            }
        }

        last = Some(num);
    }

    return true;
}
fn main() {
    let data = std::fs::read_to_string("input.txt")
        .unwrap();

    let mut num_safe = 0;
    'outer: for line in data.lines() {
        let nums = line.split_ascii_whitespace()
            .map(|word| {
                (*word)
                    .parse::<i32>()
                    .unwrap()
            })
            .collect::<Vec<i32>>(); 

        if check(&nums) {
            num_safe += 1;
        } else {
            for i in 0..nums.len() {
                let one_skipped = nums
                    .clone()
                    .into_iter()
                    .enumerate()
                    .flat_map(|(ind, num)| {
                        if i != ind {
                            Some(num)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<i32>>();
                if check(&one_skipped) {
                    num_safe += 1;
                    continue 'outer;
                }
            }
        }
    }

    println!("Number of safe lines: {num_safe}");
}