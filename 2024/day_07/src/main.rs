fn try_compute (
    current_result: i128,
    expected_result: i128,
    values: Vec<i128>
) -> bool {
    let mut values = values.clone(); 

    if values.is_empty() {
        return current_result == expected_result;
    }

    let removed = values.remove(0);

    let added = current_result + removed;
    let multiplied = current_result * removed; 
    let combined = format!("{current_result}{removed}").parse::<i128>().unwrap();
    
    try_compute ( added, expected_result, values.clone() ) || 
    try_compute ( multiplied, expected_result, values.clone() ) ||
    try_compute ( combined, expected_result, values )
}
fn main() {
    let data = std::fs::read_to_string("input.txt")
        .unwrap();

    let mut sum: i128 = 0;
    for equation_line in data.lines() {
        let result: i128 = equation_line
            .split(": ")
            .next().unwrap()
            .parse::<i128>()
            .unwrap();
        let mut numbers: Vec<i128> = equation_line
            .split(": ")
            .skip(1)
            .next().unwrap()
            .split_ascii_whitespace()
            .map(|n_st| n_st.parse::<i128>().unwrap() )
            .collect();

        println!("Expected Result: {result} ({numbers:?})");

        if try_compute ( numbers.remove(0), result, numbers ) {
            sum += result as i128;
        }
    }

    println!("Sum: {sum}");
}
