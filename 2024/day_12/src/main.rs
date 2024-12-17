struct Tile {
    value: char,
    visited: bool
}
impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!( f, "{}", self.value  )
    }
}

fn different_around (
    board: &Vec<Vec<Tile>>,
    i: usize,
    g: usize,
) -> usize {
    let mut total = 0usize;

    if i == 0                  || board[i - 1][g].value != board[i][g].value { total += 1; }
    if g == 0                  || board[i][g - 1].value != board[i][g].value { total += 1; }
    if i == board.len() - 1    || board[i + 1][g].value != board[i][g].value { total += 1; }
    if g == board[0].len() - 1 || board[i][g + 1].value != board[i][g].value { total += 1; }

    total
}
fn visit (
    board: &mut Vec<Vec<Tile>>,
    must_be: char,
    i: usize,
    g: usize,
) -> (usize, usize) {
    if board[i][g].value != must_be {
        return (0, 0);
    }
    if board[i][g].visited {
        return (0, 0);
    }
    board[i][g].visited = true;

    let mut area = 1usize;
    let mut perimeter = different_around(board, i, g);
    if i > 0 { 
        let result = visit( board, board[i][g].value, i - 1, g);
        area += result.0;
        perimeter += result.1;
    }
    if i < board.len() - 1 { 
        let result = visit( board, board[i][g].value, i + 1, g);
        area += result.0;
        perimeter += result.1;
    }
    if g > 0 { 
        let result = visit( board, board[i][g].value, i, g - 1);
        area += result.0;
        perimeter += result.1;
    }
    if g < board[0].len() - 1 { 
        let result = visit( board, board[i][g].value, i, g + 1);
        area += result.0;
        perimeter += result.1;
    }

    (area, perimeter)
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
        println!("");
    }

    let mut price = 0usize;
    for i in 0..board.len() {
        for g in 0..board[i].len() {
            if board[i][g].visited {
                continue;
            }

            let must_be = board[i][g].value;
            let (area, perimeter) = visit (
                &mut board,
                must_be,
                i, g
            );
            let tile_price = area * perimeter;

            println!("Cost for ({g}, {i}): $({tile_price})");

            price += tile_price;
        }
    }

    println!("Total price: {price}");
}
