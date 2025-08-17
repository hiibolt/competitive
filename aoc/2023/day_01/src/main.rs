use std::collections::HashSet;

fn is_up    ( ch: char ) -> bool { ['|', '7', 'F'].contains(&ch) }
fn is_down  ( ch: char ) -> bool { ['|', 'L', 'J'].contains(&ch) }
fn is_left  ( ch: char ) -> bool { ['-', 'L', 'F'].contains(&ch) }
fn is_right ( ch: char ) -> bool { ['-', '7', 'J'].contains(&ch) }

fn main() {
    // Build the map
    let mut map: Vec<Vec<char>> = {
        let input = include_str!("input.txt");
        let rows = input.split("\n");
        rows.map(|row_str| row_str.chars()
            .collect::<Vec<char>>())
            .collect()
    };

    // Find the start position
    let (start_i, start_g) = {
        let mut i = 0;
        'outer: loop {
            for j in 0..map[i].len() {
                if map[i][j] == 'S' {
                    break 'outer (i, j);
                }
            }
            if i < map.len() { i += 1; } else { panic!("No starting value!") }
        }
    };

    // Modify the S to its appropriate shape
    {
        let mut up_con = false;
        let mut down_con = false;
        let mut left_con = false;
        let mut right_con = false;

        if start_i > 0 && is_up(map[start_i - 1][start_g]) {
            up_con = true;
        }
        if start_i < map.len() - 1 && is_down(map[start_i + 1][start_g]) {
            down_con = true;
        }
        if start_g > 0 && is_left(map[start_i][start_g - 1]) {
            left_con = true;
        }
        if start_g < map[0].len() - 1 && is_right(map[start_i][start_g + 1]) {
            right_con = true;
        }

        map[start_i][start_g] = match (up_con, down_con, left_con, right_con) {
            (true, true, false, false) => '|',
            (false, false, true, true) => '-',
            (true, false, true, false) => '7',
            (true, false, false, true) => 'L',
            (false, true, true, false) => 'J',
            (false, true, false, true) => 'F',
            _ => panic!("Invalid piping structure: {map:?}\n{up_con}, {down_con}, {left_con}, {right_con}")
        };
        for row in &map {
            println!("{:?}", row);
        }
    }

    // Detect loop
    {
        let loop_cells: HashSet<(usize, usize)> = loop {
            if let Some(loop_cells) = detect_loop(&map, start_i + 1, start_g, start_i, start_g) { break loop_cells; }
            if let Some(loop_cells) = detect_loop(&map, start_i - 1, start_g, start_i, start_g) { break loop_cells; }
            panic!("No loop detected :<");
        }.into_iter().collect();
        for i in 0..map.len() {
            for g in 0..map[i].len() {
                if !loop_cells.contains(&(i, g)) {
                    map[i][g] = '.';
                }
            }
        }
    }

    // Count enclosed areas using ray casting
    let mut total_area = 0;
    for i in 0..map.len() {
        for g in 0..map[i].len() {
            if map[i][g] == '.' {
                // Cast a ray to the right and count crossings
                let mut crossings = 0;
                let mut last_bend = None;
                
                for k in (g + 1)..map[i].len() {
                    let ch = map[i][k];
                    if ch == '|' {
                        crossings += 1;
                    } else if ch == 'F' || ch == 'L' {
                        last_bend = Some(ch);
                    } else if ch == '7' {
                        if last_bend == Some('L') {
                            crossings += 1;
                        }
                        last_bend = None;
                    } else if ch == 'J' {
                        if last_bend == Some('F') {
                            crossings += 1;
                        }
                        last_bend = None;
                    }
                }
                
                if crossings % 2 == 1 {
                    total_area += 1;
                }
            }
        }
    }

    println!("Total enclosed area: {} ^^", total_area);
}
fn validate (
    map: &Vec<Vec<char>>,
    i: usize,
    g: usize
) -> Option<(usize, usize)> {
    (i != map.len() && i != usize::MAX && g != map[0].len() && g != usize::MAX)
        .then_some((i, g))
}
fn next_pair (
    map: &Vec<Vec<char>>,
    ch: char,
    i: usize,
    g: usize,
    o_i: usize,
    o_g: usize
) -> Option<(usize, usize)> {
    match ch {
        '|' => if i > o_i { validate(map, i + 1, g) } else { validate(map, i.wrapping_sub(1), g) },
        '-' => if g > o_g { validate(map, i, g + 1) } else { validate(map, i, g.wrapping_sub(1)) },
        '7' => if i < o_i { validate(map, i, g.wrapping_sub(1)) } else { validate(map, i + 1, g) },
        'F' => if i < o_i { validate(map, i, g + 1) } else { validate(map, i + 1, g) },
        'L' => if i > o_i { validate(map, i, g + 1) } else { validate(map, i.wrapping_sub(1), g) },
        'J' => if i > o_i { validate(map, i, g.wrapping_sub(1)) } else { validate(map, i.wrapping_sub(1), g) }
        _ => panic!("invalid character!")
    }
}
fn detect_loop (
    map: &Vec<Vec<char>>,
    mut i: usize,
    mut g: usize,
    start_i: usize,
    start_g: usize
) -> Option<Vec<(usize, usize)>> {
    let mut last = vec!((start_i, start_g));

    loop {
        println!("Checking: ({}, {})", i, g);
        let len = last.len() - 1;
        let next_pair = if
            ( i > last[len].0 && is_down(map[i][g]) ) ||
            ( i < last[len].0 && is_up(map[i][g]) ) ||
            ( g < last[len].1 && is_left(map[i][g]) ) ||
            ( g > last[len].1 && is_right(map[i][g]) )
        {
            next_pair(map, map[i][g], i, g, last[len].0, last[len].1)?
        } else {
            println!("Invalid entry - {}", map[i][g]);
            return None;
        };
        println!("Next pair: {:?}", next_pair);

        // Check if we looped
        if next_pair == (start_i, start_g) {
            break;
        }

        // Otherwise, add and continue
        last.push((i, g));
        i = next_pair.0;
        g = next_pair.1;
    }

    Some(last)
}