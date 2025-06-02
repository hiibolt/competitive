#[derive(Debug)]
enum Facing {
    Up,
    Down,
    Left,
    Right
}
fn rotate ( facing: Facing ) -> Facing {
    match facing {
        Facing::Up => Facing::Right,
        Facing::Right => Facing::Down,
        Facing::Down => Facing::Left,
        Facing::Left => Facing::Up
    }
}
fn has_loop (
    mut board: Vec<Vec<char>>
) -> bool {
    let mut guard_x = 0;
    let mut guard_y = 0;
    let mut facing = Facing::Up;

    for i in 0..board.len() {
        for g in 0..board[i].len() {
            match board[i][g] {
                '^' => { facing = Facing::Up; },
                'v' => { facing = Facing::Down; },
                '<' => { facing = Facing::Left; },
                '>' => { facing = Facing::Right; },
                _ => { continue; }
            }

            guard_x = g;
            guard_y = i;
            break;
        }
    }

    let mut moves = 0i32;
    loop {
        moves += 1;
        board[guard_y][guard_x] = 'X';
        match facing {
            Facing::Up => {
                if guard_y == 0 { break; }

                if board[guard_y - 1][guard_x] == '#' {
                    facing = rotate(facing);
                } else {
                    guard_y -= 1;
                }
            },
            Facing::Right => {
                if guard_x == board[0].len() - 1 { break; }

                if board[guard_y][guard_x + 1] == '#' {
                    facing = rotate(facing);
                } else {
                    guard_x += 1;
                }
            },
            Facing::Down => {
                if guard_y == board.len() - 1 { break; }

                if board[guard_y + 1][guard_x] == '#' {
                    facing = rotate(facing);
                } else {
                    guard_y += 1;
                }
            },
            Facing::Left => {
                if guard_x == 0 { break; }

                if board[guard_y][guard_x - 1] == '#' {
                    facing = rotate(facing);
                } else {
                    guard_x -= 1;
                }
            }
        }
        if moves == 200000 {
            return true;
        }
    }

    return false;
}
fn main() {
    let board: Vec<Vec<char>> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|ln| ln.chars().collect::<Vec<char>>())
        .collect();

    let mut sum = 0i32;
    for i in 0..board.len() {
        for g in 0..board[i].len() {
            if board[i][g] == '.' {
                let mut board_copy = board.clone();
                board_copy[i][g] = '#';

                if has_loop(board_copy.clone()) {
                    sum += 1;
                }
            }
        }
    }

    println!("Sum: {sum}");
}
