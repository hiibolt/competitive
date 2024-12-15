use regex::Regex;

fn main() {
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();

    let data = std::fs::read_to_string("input.txt")
        .unwrap();

    let mut sum = 0i32;

    let mut vals: Vec<(usize, i32)> = Vec::new();
    for cap in re_mul.captures_iter(&data) {
        let ch_ind = cap.iter().next().unwrap().unwrap().start();
        let (_, [n1, n2]) = cap.extract::<2>();

        vals.push(
            (
                ch_ind,
                n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap()
            )
        );
    }

    let mut dos: Vec<usize> = Vec::new();
    for cap in re_do.captures_iter(&data) {
        let ch_ind = cap.iter().next().unwrap().unwrap().start();

        dos.push(ch_ind);
    }

    let mut donts: Vec<usize> = Vec::new();
    for cap in re_dont.captures_iter(&data) {
        let ch_ind = cap.iter().next().unwrap().unwrap().start();
        
        donts.push(ch_ind);
    }

    for (ch_ind, val) in &vals {
        let closest_do: Option<&usize> = dos
            .iter()
            .rev()
            .find(|&ch_ind_inner| ch_ind_inner < ch_ind);
        let closest_dont: Option<&usize> = donts
            .iter()
            .rev()
            .find(|&ch_ind_inner| ch_ind_inner < ch_ind);

        match (closest_do, closest_dont) {
            (Some(&do_instr), Some(&dont_instr)) => {
                if do_instr > dont_instr {
                    sum += val;
                }
            },
            (Some(_), None) | (None, None) => {
                sum += val;
            },
            _ => {
                // pass
            }
        }
        println!("{:?} {:?} {val}", closest_do, closest_dont);
    }

    println!("Sum: {sum}");
}


