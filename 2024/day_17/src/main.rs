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
fn main() {
    let data = std::fs::read_to_string("input.txt")
        .unwrap();

    // Extract registers A-C
    let mut a: u128 = data.lines()
        .next().unwrap()
        .split("Register A: ")
        .nth(1).unwrap()
        .parse::<u128>().unwrap();
    let mut b: u128 = data.lines()
        .nth(1).unwrap()
        .split("Register B: ")
        .nth(1).unwrap()
        .parse::<u128>().unwrap();
    let mut c: u128 = data.lines()
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

    let mut instruction_ind = 0usize;
    let mut output = Vec::new();
    while instruction_ind < program.len() {
        let opcode = program[instruction_ind];
        let operand = program[instruction_ind + 1];
        println!("Handling opcode `{opcode}` for operand `{operand}`");

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
            }
            _ => panic!("Invalid opcode!")
        }

        instruction_ind += 2;
    }

    let output_str = output.join(",");

    println!("\nFinal State:\nA: {a}\nB: {b}\nC: {c}");
    println!("Program: {program:?}\n");
    println!("Final output: {output_str}");
}
