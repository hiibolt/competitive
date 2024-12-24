use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
enum GateType {
    AND,
    OR,
    XOR
}
#[derive(Debug, Clone)]
struct Gate<'a> {
    target_1: &'a str,
    target_2: &'a str,
    result:   &'a str,
    gate:     GateType
}
fn find_gate<'a> (
    gates: &Vec<Gate<'a>>,
    target_1: String,
    target_2: String,
    gate_type: GateType
) -> Option<Gate<'a>> {
    for gate in gates {
        if (gate.target_1 == &target_1 || gate.target_2 == &target_1) && 
           (gate.target_1 == &target_2 || gate.target_2 == &target_2) &&
           gate.gate == gate_type
        {
            return Some(gate.clone());
        }
    }

    None
}
fn main() {
    let gate_re = Regex::new(r"(.+) (OR|AND|XOR) (.+) -> (.+)")
        .unwrap();

    let data = std::fs::read_to_string("input.txt")
        .unwrap();
    /*
    let connections: HashMap<&str, bool> = data
        .split("\n\n")
        .next().unwrap()
        .lines()
        .fold(HashMap::new(), |mut hm, ln| {
            let name = ln
                .split(": ")
                .next().unwrap();
            let num: bool = if ln.split(": ").nth(1).unwrap() == "1" {
                true
            } else {
                false
            };

            hm.insert(name, num);

            hm
        }); */
    let mut gates = Vec::new();
    let gates_input = data
        .split("\n\n")
        .nth(1).unwrap();
    for (_, [target_1, gate, target_2, result]) in gate_re
        .captures_iter(gates_input)
        .map(|c| c.extract())
    {
        gates.push(Gate {
            target_1,
            target_2,
            result,
            gate: if gate == "OR" {
                GateType::OR
            } else if gate == "AND" {
                GateType::AND
            } else {
                GateType::XOR
            }
        });
    }
    //println!("{gates:?}");

    let mut cin = find_gate(&gates, 
        String::from("x00"),
        String::from("y00"),
        GateType::AND
    ).unwrap().result;
    for i in 1..=44 {
        println!("Made it to operation #{i:02}");
        let x_reg = format!("x{i:02}");
        let y_reg = format!("y{i:02}");

        let sum_1 = find_gate(&gates, 
            x_reg.clone(), 
            y_reg.clone(), 
            GateType::XOR
        ).expect("Invalid SUM_1 operation!").result;
        println!("`sum-1`: {sum_1}");

        let output = find_gate(&gates, 
            sum_1.to_string(), 
            cin.to_string(), 
            GateType::XOR
        ).expect("No valid output operation!").result;
        assert_eq!(output, &format!("z{i:02}"));

        let carry_1 = find_gate(&gates, 
            sum_1.to_string(), 
            cin.to_string(), 
            GateType::AND
        ).expect("No valid `carry_1` operation!").result;
        println!("`carry_1`: {carry_1}");

        let carry_2 = find_gate(&gates, 
            x_reg, 
            y_reg, 
            GateType::AND
        ).expect("No valid `carry_2` operation!").result;
        println!("`carry_2`: {carry_2}");

        cin = find_gate(&gates, 
            carry_1.to_string(), 
            carry_2.to_string(), 
            GateType::OR
        ).expect("No valid `c-out` operation!").result;
        println!("New `cin`: {cin}");
    }

    // pair 1: kwb and z12
    // pair 2: qkf and z16
    // pair 3: tgr and z24
    // pair 4: jqn and cph
    // cph,jqn,kwb,qkf,tgr,z12,z16,z24

    println!("Success ;3");
}