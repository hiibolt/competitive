use regex::Regex;

fn calculate_cost ( 
    a1: i128,
    a2: i128,
    b1: i128,
    b2: i128,
    r1: i128,
    r2: i128
) -> Option<i128> {
    // Calculate A
    let numerator = r1 * b2 - b1 * r2;
    let denominator = a1 * b2 - b1 * a2;

    if denominator == 0 { return None; }
    if numerator % denominator != 0 { return None; }

    let a = numerator / denominator;

    // Calculate b
    let numerator = r2 - a2 * a;
    let denominator = b2;

    if denominator == 0 { return None; }
    if numerator % denominator != 0 { return None; }

    let b = numerator / denominator;

    Some(3 * a + b)
}
fn main() {
    let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();
    let data = std::fs::read_to_string("input.txt")
        .unwrap();

    let fewest_tokens = re.captures_iter(&data)
        .map(|c| c.extract())
        .flat_map(|(_, [a1, a2, b1, b2, r1, r2])| {
            let a1 = a1.parse::<i128>().unwrap();
            let a2 = a2.parse::<i128>().unwrap();
            let b1 = b1.parse::<i128>().unwrap();
            let b2 = b2.parse::<i128>().unwrap();
            let r1 = r1.parse::<i128>().unwrap() + 10000000000000;
            let r2 = r2.parse::<i128>().unwrap() + 10000000000000;

            println!("{} {} {} {} = {} {}", a1, a2, b1, b2, r1, r2);

            let cost = calculate_cost(a1,a2,b1,b2,r1,r2);
            println!("Cost: {cost:?}");

            cost
        })
        .sum::<i128>();

    println!("Fewest tokens: {fewest_tokens}");
}
