use std::collections::VecDeque;

struct Tile {
    value: char,
    visited: bool
}
impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!( f, "{}", self.value  )
    }
}

fn main() {
    let mut board = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|ln| {
            ln.chars()
                .map(|ch| Tile { value: ch, visited: false })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>();

    for row in &board {
        for val in row {
            print!("{val}");
        }
        println!();
    }

    let mut price = 0usize;
    for i in 0..board.len() {
        for g in 0..board[i].len() {
            if board[i][g].visited {
                continue;
            }

            // Build the board
            let mut visual_board: Vec<Vec<char>> = Vec::new();
            for inner_i in 0..board.len() {
                let length = board[inner_i].len() * 2 + 2;
                let visual_board_row = vec!(' '; length);

                if inner_i == 0 { visual_board.push(visual_board_row.clone()); }
                visual_board.push(visual_board_row.clone());
                visual_board.push(visual_board_row);
            }

            // Add nodes
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            queue.push_back((i, g));
            while let Some(node) = queue.pop_front() {
                board[node.0][node.1].visited = true;
                visual_board[node.0 * 2 + 1][node.1 * 2 + 1] = board[node.0][node.1].value;

                if node.0 > 0 && 
                    !board[node.0 - 1][node.1].visited &&
                    board[node.0 - 1][node.1].value == board[node.0][node.1].value
                {
                    queue.push_back((node.0 - 1, node.1));
                }
                if node.0 < board.len() - 1 && 
                    !board[node.0 + 1][node.1].visited &&
                    board[node.0 + 1][node.1].value == board[node.0][node.1].value
                {
                    queue.push_back((node.0 + 1, node.1));
                }
                if node.1 > 0 && 
                    !board[node.0][node.1 - 1].visited &&
                    board[node.0][node.1 - 1].value == board[node.0][node.1].value
                {
                    queue.push_back((node.0, node.1 - 1));
                }
                if node.1 < board[0].len() - 1 && 
                    !board[node.0][node.1 + 1].visited &&
                    board[node.0][node.1 + 1].value == board[node.0][node.1].value
                {
                    queue.push_back((node.0, node.1 + 1));
                }
            }

            // Add fences
            for j in 0..visual_board.len() {
                for k in 0..visual_board[i].len() {
                    match visual_board[j][k] {
                        ' ' | '-' | '|' => { continue; },
                        _ => { }
                    }

                    if k == 1 || ( k > 1 && visual_board[j][k - 2] != visual_board[j][k] ) {
                        visual_board[j][k - 1] = '|';
                    }
                    if k == visual_board[0].len() - 2 ||
                        ( k < visual_board[0].len() - 2 && 
                          visual_board[j][k + 2] != visual_board[j][k] )
                    {
                        visual_board[j][k + 1] = '|';
                    }
                    if j == 1 || ( j > 1 && visual_board[j - 2][k] != visual_board[j][k] ) {
                        visual_board[j - 1][k] = '-';
                    }
                    if j == visual_board.len() - 2 ||
                        ( j < visual_board.len() - 2 && 
                          visual_board[j + 2][k] != visual_board[j][k] )
                    {
                        visual_board[j + 1][k] = '-';
                    }
                }
            }

            let mut edges = 0usize;

            // Calculate horizontal edges
            let mut j = 0usize;
            let mut k;
            while j < visual_board.len() {
                k = 1;
                while k < visual_board[0].len() {
                    if visual_board[j][k] == '-' {
                        edges += 1;
                        while k < visual_board[j].len() - 3 &&
                                visual_board[j][k + 2] == '-' &&
                                !((j > 0 && visual_board[j-1][k+1] == '|') || 
                                  (j < visual_board.len() - 1 && visual_board[j+1][k+1] == '|'))
                        {
                            k += 2;
                        }
                    }
                    k += 2;
                }
                j += 1;
            }

            // Calculate vertical edges
            let mut j;
            let mut k = 0usize;
            while k < visual_board.len() {
                j = 1;
                while j < visual_board.len() {
                    if visual_board[j][k] == '|' {
                        edges += 1;
                        while j < visual_board.len() - 3 &&
                                visual_board[j + 2][k] == '|' &&
                                !((k > 0 && visual_board[j+1][k-1] == '-') || 
                                  (k < visual_board[0].len() - 1 && visual_board[j+1][k+1] == '-'))
                        {
                            j += 2;
                        }
                    }
                    j += 2;
                }
                k += 1;
            }

            // Calculate area
            let area = visual_board.iter()
                .map(|row| {
                    row.iter()
                        .filter(|&&e| e != '-' && e != ' ' && e != '|')
                        .count()
                })
                .sum::<usize>();

            println!("Visual board (with fences):");
            for row in &visual_board {
                print!("|");
                for val in row {
                    print!("{val}");
                }
                println!("|");
            }
            println!("Number of edges: {edges}\nArea: {area}\nPrice: {}", area * edges);

            price += area * edges;
        }
    }

    println!("Total price: {price}");
}
