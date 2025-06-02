fn valid_coord (
    board: &Vec<Vec<char>>,
    x: i32,
    y: i32
) -> bool {
    x > -1 && y > -1 && y < board.len() as i32 && x < board[0].len() as i32
}
fn main() {
    let board = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|ln| ln.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut final_board = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|ln| ln.chars().map(|_| '.').collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for i in 0..board.len() {
        for g in 0..board[i].len() {
            let freq = board[i][g];

            if freq == '.' || freq == '#' { continue; }

            for j in 0..board.len() {
                for k in 0..board[j].len() {
                    if ( i == j && k == g ) || board[j][k] != freq { continue; }

                    let x_diff = k as i32 - g as i32;
                    let y_diff = j as i32 - i as i32;

                    println!("found match: ({g}, {i}) and ({k}, {j})");
                    println!("with an offset of ({x_diff}, {y_diff})");

                    final_board[i][g] = '#';

                    let mut left_x_proj = g as i32 - x_diff;
                    let mut left_y_proj = i as i32 - y_diff;

                    while valid_coord(&board, left_x_proj, left_y_proj) {
                        if final_board[left_y_proj as usize][left_x_proj as usize] == '.' {
                            final_board[left_y_proj as usize][left_x_proj as usize] = '#';
                            println!("Placed at ({left_x_proj}, {left_y_proj})");
                        }
                        left_x_proj -= x_diff;
                        left_y_proj -= y_diff;
                    }

                    let mut right_x_proj = k as i32 + x_diff;
                    let mut right_y_proj = j as i32 + y_diff;

                    while valid_coord(&board, right_x_proj, right_y_proj) {
                        if final_board[right_y_proj as usize][right_x_proj as usize] == '.' {
                            final_board[right_y_proj as usize][right_x_proj as usize] = '#';
                            println!("Placed at ({right_x_proj}, {right_y_proj})");
                        }
                        right_x_proj += x_diff;
                        right_y_proj += y_diff;
                    }
                }
            }
        }
    }

    let mut sum = 0i32;
    for row in &final_board {
        for ch in row {
            if *ch == '#' {
                sum += 1;
            }
        }
        println!("{row:?}");
    }

    println!("Sum: {sum}");
}
