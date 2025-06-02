use std::collections::{HashMap, BTreeSet};

fn find_max<'a> (
    current:   BTreeSet<&'a str>,
    computers: &BTreeSet<&'a str>,
    network:   &HashMap<&'a str, BTreeSet<&'a str>>,

    cache: &mut HashMap<BTreeSet<&'a str>, BTreeSet<&'a str>>
) -> BTreeSet<&'a str> {
    if (*cache).contains_key(&current) {
        return cache[&current].clone();
    }

    let mut ret = current.clone();

    'outer: for &target_computer in computers {
        if current.contains(target_computer) {
            continue;
        }

        for &current_computer in &current {
            if !network[current_computer].contains(target_computer) {
                continue 'outer;
            }
        }

        let mut current_cloned = current.clone();
        current_cloned.insert(&target_computer);

        let check_higher = find_max(
            current_cloned,
            computers,
            network,
            cache
        );
        if check_higher.len() > ret.len() {
            ret = check_higher;
        }
    }

    cache.insert(current, ret.clone());

    ret
}
fn main() {
    let data = std::fs::read_to_string("input.txt")
        .unwrap();
    let links = data
        .lines()
        .map(|ln| ln.split("-").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut network: HashMap<&str, BTreeSet<&str>> = HashMap::new();
    let mut computers = BTreeSet::new();
    for link in links {
        computers.insert(link[0]);
        computers.insert(link[1]);
        network.entry(&link[0]).or_insert(BTreeSet::new())
            .insert(link[1]);
        network.entry(&link[1]).or_insert(BTreeSet::new())
            .insert(link[0]);
    }

    let mut cache = HashMap::new();
    let mut max = find_max ( BTreeSet::new(), &computers, &network, &mut cache )
        .into_iter()
        .collect::<Vec<&str>>();
    max.sort_by_key(|st| {
        format!(
            "{:02}{:02}",
            ((st.chars().next().unwrap() as u32) - 97),
            ((st.chars().nth(1).unwrap() as u32) - 97)
        ).parse::<u32>().unwrap()
    });

    let password = max.join(",");

    println!("The password is: {password:?}");
}
