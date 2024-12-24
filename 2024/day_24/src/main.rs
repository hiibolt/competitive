use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
enum GateType {
    AND,
    OR,
    XOR
}
#[derive(Debug)]
struct Gate<'a> {
    target_1: &'a str,
    target_2: &'a str,
    result:   &'a str,
    gate:     GateType
}
fn main() {
    let gate_re = Regex::new(r"(.+) (OR|AND|XOR) (.+) -> (.+)")
        .unwrap();

    let data = std::fs::read_to_string("input.txt")
        .unwrap();
    let mut connections: HashMap<&str, bool> = data
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
        });
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

    while !gates.is_empty() {
        gates = gates.into_iter()
            .filter(|gate| {
                if !connections.contains_key(gate.target_1) ||
                   !connections.contains_key(gate.target_2) {
                    return true;
                }

                let target_1_val = connections[gate.target_1];
                let target_2_val = connections[gate.target_2];

                match gate.gate {
                    GateType::AND => {
                        connections.insert(gate.result, target_1_val && target_2_val);
                    },
                    GateType::OR => {
                        connections.insert(gate.result, target_1_val || target_2_val);
                    },
                    GateType::XOR => {
                        connections.insert(gate.result, target_1_val ^ target_2_val);
                    }
                }

                false
            })
            .collect();
    }

    let mut z_wires: Vec<(&str, bool)> = connections.into_iter()
        .filter(|(wire, _)| wire.starts_with("z"))
        .collect();
    z_wires.sort_by_key(|(wire, _)| format!(
        "{:02}{:02}",
        wire.chars().nth(1).unwrap() as u32 - 48,
        wire.chars().nth(2).unwrap() as u32 - 48
    ).parse::<u32>().unwrap());

    let mut ret = 0usize;
    for (ind, (_, val)) in z_wires.into_iter().enumerate() {
        if val {
            ret += 2usize.pow(ind as u32);
        }
    }

    println!("Final number: {ret}");
}
