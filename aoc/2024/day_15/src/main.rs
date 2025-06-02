fn print ( map: &Vec<Vec<char>> ) {
    for row in map {
        for node in row {
            print!("{node}");
        }
        println!();
    }
}
fn main() {
    let data = std::fs::read_to_string("input.txt")
        .unwrap();

    let moves = data.split("\n\n")
        .nth(1).unwrap()
        .lines()
        .flat_map(|ln| ln.chars().collect::<Vec<char>>() )
        .collect::<Vec<char>>();
    let mut map = data.split("\n\n")
        .next().unwrap()
        .lines()
        .map(|ln| {
            ln.chars()
                .flat_map(|ch| {
                    match ch {
                        '.' => vec!('.', '.'),
                        '#' => vec!('#', '#'),
                        'O' => vec!('[', ']'),
                        '@' => vec!('@', '.'),
                        _ => vec!()
                    }
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    print(&map);
    'move_loop: for mv in &moves {
        match mv {
            '<' => {
                'outer: for i in 0..map.len() {
                    for g in 0..map[i].len() {
                        if map[i][g] != '@' { continue; }

                        if g == 0 { 
                            continue 'move_loop;
                        }

                        let left = map[i][g - 1];
                        match left {
                            '.' => {
                                map[i][g] = '.';
                                map[i][g - 1] = '@';
                            },
                            ']' => {
                                for search_g in 0..(g - 3) {
                                    let search_g = g - search_g - 3;

                                    println!("Checking ({i}, {search_g}) ({})", map[i][search_g]);

                                    if map[i][search_g] == '#' {
                                        break;
                                    }
                                    if map[i][search_g] == '.' {
                                        for move_g in search_g..(g - 1) {
                                            map[i][move_g] = map[i][move_g + 1];
                                        }
                                        map[i][g - 1] = '@';
                                        map[i][g] = '.';
                                        break;
                                    }
                                }
                            },
                            _ => {}
                        }

                        break 'outer;
                    }
                }
            },
            '>' => {
                'outer: for i in 0..map.len() {
                    for g in 0..map[i].len() {
                        if map[i][g] != '@' { continue; }

                        if g == map[i].len() - 1 { 
                            continue 'move_loop;
                        }

                        let right = map[i][g + 1];
                        println!("Character on right: {right}");
                        match right {
                            '.' => {
                                map[i][g] = '.';
                                map[i][g + 1] = '@';
                            },
                            '[' => {
                                for search_g in (g + 3)..(map[i].len() - 1) {
                                    println!("Checking ({i}, {search_g}) ({})", map[i][search_g]);

                                    if map[i][search_g] == '#' {
                                        break;
                                    }
                                    if map[i][search_g] == '.' {
                                        for move_g in ((g + 1)..=search_g).rev() {
                                            map[i][move_g ] = map[i][move_g - 1];
                                        }
                                        map[i][g + 1] = '@';
                                        map[i][g] = '.';
                                        break;
                                    }
                                }
                            },
                            _ => {}
                        }

                        break 'outer;
                    }
                }
            },
            '^' => {
                'outer: for i in 0..map.len() {
                    for g in 0..map[i].len() {
                        if map[i][g] != '@' { continue; }

                        if i == 0 {
                            continue 'move_loop;
                        }

                        let up = map[i - 1][g];
                        println!("Character up: {up}");
                        match up {
                            '.' => {
                                map[i][g] = '.';
                                map[i - 1][g] = '@';
                            },
                            ']' | '[' => {
                                let mut ind = 0;
                                let mut can_move = true;
                                let mut nodes: Vec<(usize, usize)> = Vec::new();
                                nodes.push((i - 1, g));
                                if up == ']' {
                                    nodes.push((i - 1, g - 1));
                                } else {
                                    nodes.push((i - 1, g + 1));
                                }

                                while can_move && ind < nodes.len() {
                                    let (node_i, node_g) = nodes[ind];

                                    println!("Checking above ({}, {}) ({})", node_g, node_i, map[node_i - 1][node_g]);

                                    if node_i == 0 {
                                        can_move = false;
                                        break;
                                    }
                                    
                                    match map[node_i - 1][node_g]  {
                                        '#' => {
                                            can_move = false;
                                            break;
                                        },
                                        '[' => {
                                            if !nodes.contains(&(node_i - 1, node_g)) {
                                                nodes.push((node_i - 1, node_g));

                                            }
                                            if !nodes.contains(&(node_i - 1, node_g + 1)) {
                                                nodes.push((node_i - 1, node_g + 1));
                                            }
                                        },
                                        ']' => {
                                            if !nodes.contains(&(node_i - 1, node_g)) {
                                                nodes.push((node_i - 1, node_g));

                                            }
                                            if !nodes.contains(&(node_i - 1, node_g - 1)) {
                                                nodes.push((node_i - 1, node_g - 1));
                                            }
                                        },
                                        _ => {}
                                    }

                                    ind += 1;
                                }

                                println!("Could move? {can_move} {:?}", nodes);

                                if can_move {
                                    for ind in (0..nodes.len()).rev() {
                                        let (node_i, node_g) = nodes[ind];
    
                                        map[node_i - 1][node_g] = map[node_i][node_g];
                                        map[node_i][node_g] = '.';
                                        println!("{ind}");
                                    }

                                    map[i][g] = '.';
                                    map[i - 1][g] = '@';
                                }
                            }
                            _ => {}
                        }

                        break 'outer;
                    }
                }
            },
            'v' => {
                'outer: for i in 0..map.len() {
                    for g in 0..map[i].len() {
                        if map[i][g] != '@' { continue; }

                        if i == map.len() - 1 {
                            continue 'move_loop;
                        }

                        let down = map[i + 1][g];
                        //println!("Character down: {down}");
                        match down {
                            '.' => {
                                map[i][g] = '.';
                                map[i + 1][g] = '@';
                            },
                            ']' | '[' => {
                                let mut ind = 0;
                                let mut can_move = true;
                                let mut nodes: Vec<(usize, usize)> = Vec::new();
                                nodes.push((i + 1, g));
                                if down == ']' {
                                    nodes.push((i + 1, g - 1));
                                } else {
                                    nodes.push((i + 1, g + 1));
                                }

                                while can_move && ind < nodes.len() {
                                    let (node_i, node_g) = nodes[ind];

                                    println!("Checking below ({}, {}) ({})", node_g, node_i, map[node_i + 1][node_g]);

                                    if node_i == map.len() - 1 {
                                        can_move = false;
                                        break;
                                    }
                                    
                                    match map[node_i + 1][node_g]  {
                                        '#' => {
                                            can_move = false;
                                            break;
                                        },
                                        '[' => {
                                            if !nodes.contains(&(node_i + 1, node_g)) {
                                                nodes.push((node_i + 1, node_g));

                                            }
                                            if !nodes.contains(&(node_i + 1, node_g + 1)) {
                                                nodes.push((node_i + 1, node_g + 1));
                                            }
                                        },
                                        ']' => {
                                            if !nodes.contains(&(node_i + 1, node_g)) {
                                                nodes.push((node_i + 1, node_g));

                                            }
                                            if !nodes.contains(&(node_i + 1, node_g - 1)) {
                                                nodes.push((node_i + 1, node_g - 1));
                                            }
                                        },
                                        _ => {}
                                    }

                                    ind += 1;
                                }

                                //println!("Could move? {can_move} {:?}", nodes);

                                if can_move {
                                    for ind in (0..nodes.len()).rev() {
                                        let (node_i, node_g) = nodes[ind];
    
                                        map[node_i + 1][node_g] = map[node_i][node_g];
                                        map[node_i][node_g] = '.';
                                        //println!("{ind}");
                                    }

                                    map[i][g] = '.';
                                    map[i + 1][g] = '@';
                                }
                            }
                            _ => {}
                        }

                        break 'outer;
                    }
                }
            }
            _ => { println!("^^'"); }
        }

        //print(&map);

    }
    let mut gps_coordinate_sum = 0usize;
    for (i, row) in map.iter().enumerate() {
        for (g, node) in row.iter().enumerate() {
            if *node != '[' {
                continue;
            }

            gps_coordinate_sum += 100 * i + g;
        }
    }

    println!("Final GPS coordinate sum: {gps_coordinate_sum}");
}
