use memoize::memoize;

#[memoize]
fn possible (
    request: String,
    towels: Vec<String>
) -> usize {
    //println!("Checking `{request}`");

    if request.len() == 0 {
        return 1;
    }

    let mut ret = 0usize;
    for towel in &towels {
        if let Some(stripped) = request.strip_prefix(towel) {
            ret += possible( stripped.to_string(), towels.clone() );
        }
    }

    ret
}
fn main() {
    let data = std::fs::read_to_string("input.txt")
        .unwrap();

    let towels = data
        .split("\n\n")
        .next().unwrap()
        .split(", ")
        .map(|st| st.to_string())
        .collect::<Vec<String>>();
    let requests = data
        .split("\n\n")
        .nth(1).unwrap()
        .lines()
        .map(|st| st.to_string())
        .collect::<Vec<String>>();

    println!("Towels: {towels:?}");
    println!("Requests: {requests:?}");

    let mut num_possible_combinations = 0usize;
    for request in &requests {
        num_possible_combinations += possible(request.to_string(), towels.clone());
    }

    println!("Total number possible: {num_possible_combinations}");
}
