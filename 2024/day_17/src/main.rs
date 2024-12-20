use std::sync::{ Arc, Mutex };

fn literal_operand ( operand: char ) -> u128 {
    operand.to_digit(10).unwrap() as u128
}
fn combo_operand ( 
    operand: char,
    a: u128,
    b: u128,
    c: u128
) -> u128 {
    match operand {
        '0' | '1' | '2' | '3' => operand.to_digit(10).unwrap() as u128,
        '4' => a,
        '5' => b,
        '6' => c,
        _ => panic!("Invalid operand!")
    }
}
fn process_program(
    mut a: u128,
    mut b: u128,
    mut c: u128,
    program: &Vec<char>
) -> String {
    let mut instruction_ind = 0usize;
    let mut output = Vec::new();
    while instruction_ind < program.len() {
        let opcode = program[instruction_ind];
        let operand = program[instruction_ind + 1];

        match opcode {
            '0' | '6' | '7' => { // Division
                let numerator = a;
                let denominator = 2u128.pow(combo_operand(operand, a, b, c) as u32);

                match opcode {
                    '0' => {
                        a = numerator / denominator;
                    },
                    '6' => {
                        b = numerator / denominator;
                    },
                    '7' => {
                        c = numerator / denominator;
                    },
                    _ => unreachable!()
                }
                
            },
            '1' => { // Bitwise XOR (B and literal operand)
                b = b ^ literal_operand(operand);
            },
            '2' => { // Modulo
                b  = combo_operand(operand, a, b, c) % 8;
            },
            '3' => { // Jump
                if a != 0 {
                    instruction_ind = literal_operand(operand) as usize;
                    continue;
                }
            },
            '4' => { // Bitwise XOR (B and C)
                b = b ^ c;
            },
            '5' => { // Output
                output.push((combo_operand(operand, a, b, c) % 8).to_string());
                if output.len() > program.len() ||
                    output[output.len() - 1].chars().next().unwrap() != program[output.len() - 1]
                {
                    return String::new();
                }
            }
            _ => panic!("Invalid opcode!")
        }

        instruction_ind += 2;
    }

    output.join(",")
}
fn main() {
    let data = std::fs::read_to_string("input.txt")
        .unwrap();

    // Extract registers A-C
    let a: u128 = data.lines()
        .next().unwrap()
        .split("Register A: ")
        .nth(1).unwrap()
        .parse::<u128>().unwrap();
    let b: u128 = data.lines()
        .nth(1).unwrap()
        .split("Register B: ")
        .nth(1).unwrap()
        .parse::<u128>().unwrap();
    let c: u128 = data.lines()
        .nth(2).unwrap()
        .split("Register C: ")
        .nth(1).unwrap()
        .parse::<u128>().unwrap();

    // Extract program instructions
    let program = data.lines()
        .skip(3)
        .nth(1).unwrap()
        .split("Program: ")
        .nth(1).unwrap()
        .split(',')
        .map(|st| st.chars().next().unwrap() )
        .collect::<Vec<char>>();

    println!("A: {a}\nB: {b}\nC: {c}");
    println!("Program: {program:?}");

    let output_str = process_program(a, b, c, &program);
    let program_str = program.iter()
        .map(|ch| ch.to_string())
        .collect::<Vec<String>>()
        .join(",");

    println!("Final output: {output_str}");
    println!("Original program: {program_str}");

    let threads = 500;
    let mut handles = Vec::new();
    let total = Arc::new(Mutex::new(Vec::new()));
    for i in 0..threads {
        let total_handle = total.clone();
        let program_copy = program.clone();
        let program_str_copy = program_str.clone();
        handles.push(std::thread::spawn( move || {
            let start = (1000000000000 / threads) * i;
            let end = (1000000000000 / threads) * (i + 1);
            for i in start..end {
                if i == a { continue; }
                if process_program(i as u128, b, c, &program_copy) == program_str_copy {
                    println!("A copy exists when register A is {i}.");
                    // Add it to the global total
                    total_handle.lock().expect("Failed to lock global total!").push(i);
                    break;
                }
            }

        }));
    }

    println!("\n[ Successfully launched {threads} threads ]");

    for handle in handles {
        handle.join().expect("Thread join failed!");
    }

    let final_total = total.lock().expect("Failed to lock final value!").clone();
    if let Some(lowest) = final_total
        .iter()
        .min() 
    {
        println!("Lowest: {lowest:?}");
    }
}
