use std::collections::{HashMap, HashSet};

fn main() {
    let data = std::fs::read_to_string("input.txt")
        .unwrap();
    let links = data
        .lines()
        .map(|ln| ln.split("-").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut network: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut computers = HashSet::new();
    for link in links {
        computers.insert(link[0]);
        computers.insert(link[1]);
        network.entry(&link[0]).or_insert(HashSet::new())
            .insert(link[1]);
        network.entry(&link[1]).or_insert(HashSet::new())
            .insert(link[0]);
    }

    /*
    let mut total = 0usize;
    for i in 0..computers.len() - 2 {
        for j in (i + 1)..computers.len() - 1 {
            for k in (j + 1)..computers.len() {
                if !computers[i].starts_with("t") && 
                   !computers[j].starts_with("t") && 
                   !computers[k].starts_with("t")
                {
                    continue;
                }
                if !network[computers[i]].contains(computers[j]) || 
                   !network[computers[i]].contains(computers[k])
                {
                    continue;
                }
                if !network[computers[j]].contains(computers[i]) || 
                   !network[computers[j]].contains(computers[k])
                {
                    continue;
                }
                if !network[computers[k]].contains(computers[i]) || 
                   !network[computers[k]].contains(computers[j])
                {
                    continue;
                }
                println!("{} {} {}", computers[i], computers[j], computers[k]);
                total += 1;
            }
        }
    } */

    let total = find_max(vec!(), computers, &network);
    println!("Total: {total:?}");
}
