fn is_mas ( st: &str ) -> bool {
    st == "MAS" || st == "SAM"
}
fn main() {
    let board: Vec<Vec<char>> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|ln| ln.chars().collect::<Vec<char>>())
        .collect();

    let mut sum = 0i32;
    for i in 0..=(board.len() - 3) {
        for g in 0..=(board[i].len() - 3) {
            let down_right_diag = &format!(
                "{}{}{}",
                board[i][g],
                board[i + 1][g + 1],
                board[i + 2][g + 2]
            );
            let left_up_diag = &format!(
                "{}{}{}",
                board[i + 2][g],
                board[i + 1][g + 1],
                board[i][g + 2]
            );

            if is_mas(&down_right_diag) && is_mas(&left_up_diag) {
                sum += 1;
            }
        }
    }

    println!("{board:?}\nSum: {sum}");
}