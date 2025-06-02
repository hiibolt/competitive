fn combo_operand ( 
    operand: u128,
    a: u128,
    b: u128,
    c: u128
) -> u128 {
    match operand {
        0..=3 => operand,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Invalid operand!")
    }
}
fn process_program(
    program: Vec<u128>,
    outputs: &[u128],
    ans: u128
) -> Option<u128> {
    if outputs.is_empty() {
        return Some(ans);
    }
    for i in 0..8 {
        let mut a = ans << 3 | i;
        let mut b = 0u128;
        let mut c = 0u128;
        let mut output: Option<u128> = None; 

        for instruction_ind in (0..(program.len() - 2)).step_by(2) {
            let opcode = program[instruction_ind];
            let operand = program[instruction_ind + 1];

            match opcode {
                0 | 6 | 7 => { // Division
                    let numerator = a;
                    let denominator = combo_operand(operand, a, b, c);

                    match opcode {
                        0 => {
                            //a = numerator >> denominator;
                        },
                        6 => {
                            b = numerator >> denominator;
                        },
                        7 => {
                            c = numerator >> denominator;
                        },
                        _ => unreachable!()
                    }
                    
                },
                1 => { // Bitwise XOR (B and literal operand)
                    b ^= operand;
                },
                2 => { // Modulo
                    b  = combo_operand(operand, a, b, c) % 8;
                },
                3 => { // Jump
                    panic!("Jump wasn't at the end!");
                },
                4 => { // Bitwise XOR (B and C)
                    b ^= c;
                },
                5 => { // Output
                    output = Some(combo_operand(operand, a, b, c) % 8);
                }
                _ => panic!("Invalid opcode!")
            }
        

        
            if output == Some(outputs[outputs.len() - 1]) {
                let result = process_program(
                    program.clone(),
                    &outputs[0..outputs.len()-1],
                    a
                );

                if result.is_some() {
                    return result;
                }
            }
        }
    }
    None
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
        .map(|st| st.chars().next().unwrap().to_digit(10).unwrap() as u128 )
        .collect::<Vec<u128>>();

    println!("A: {a}\nB: {b}\nC: {c}");
    println!("Program: {program:?}");

    let solution = process_program(program.clone(), &program, 0);


    println!("Solution: {solution:?}");
}
