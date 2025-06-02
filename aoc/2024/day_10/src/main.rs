fn get_score ( 
    board: &Vec<Vec<u32>>,
    i: usize,
    g: usize,
    //visited_nines: &mut Vec<(usize, usize)>
) -> u32 {
    println!("Navigating ({g}, {i})");
    if board[i][g] == 9 {
        //if !visited_nines.contains(&(g, i)) {
        //    visited_nines.push((g, i));
            return 1;
        //}
    }

    let mut sum = 0u32;
    if i > 0 && board[i - 1][g] == board[i][g] + 1 {
        //sum += get_score( board, i - 1, g, visited_nines );
        sum += get_score( board, i - 1, g );
    }
    if i < board.len() - 1 && board[i + 1][g] == board[i][g] + 1 {
        //sum += get_score( board, i + 1, g, visited_nines );
        sum += get_score( board, i + 1, g );
    }
    if g > 0 && board[i][g - 1] == board[i][g] + 1 {
        //sum += get_score( board, i, g - 1, visited_nines );
        sum += get_score( board, i, g - 1 );
    }
    if g < board[0].len() - 1 && board[i][g + 1] == board[i][g] + 1 {
        //sum += get_score( board, i, g + 1, visited_nines );
        sum += get_score( board, i, g + 1 );
    }

    sum
}
fn main() {
    let board = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|ln| ln.chars().map(|ch| {
            ch.to_digit(10).unwrap()
        }).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    let mut sum = 0u32;
    for i in 0..board.len() {
        for g in 0..board[i].len() {
            if board[i][g] == 0 {
                //let mut visited = Vec::new();
                let score = get_score(
                    &board,
                    i,
                    g,
                    //visited.as_mut()
                );

                println!("Trailhead at ({g}, {i}) with a score of {score}");

                sum += score;
            }
        }
    }

    for row in &board {
        println!("{row:?}");
    }

    println!("Sum: {sum}");
}
