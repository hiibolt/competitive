const MOVEABLES: [char; 2] = ['E', '.'];

#[derive(Debug, PartialEq, Clone)]
enum Facing {
    North,
    East,
    South,
    West
}
fn navigate (
    lowest_score: &mut Option<usize>,
    map: &Vec<Vec<char>>,
    score: usize,
    i: usize,
    g: usize,
    facing: Facing,
    mut visited: Vec<(usize, usize)>
) {
    //println!("Navigated to ({g}, {i}) ({})", map[i][g]);

    if map[i][g] == 'E' {
        println!("Hit `E` with a score of {score}");
        if let Some(lowest_score) = lowest_score {
            if score < *lowest_score {
                *lowest_score = score;
            }
        } else {
            *lowest_score = Some(score);
        }
        return;
    }

    if let Some(lowest_score) = lowest_score {
        if score >= *lowest_score {
            //println!("Score ({score}) impossibly high, aborting");
            return;
        }
    }

    visited.push((i, g));
    if i != 0 && !visited.contains(&(i - 1, g)) && MOVEABLES.contains(&map[i - 1][g]) {
        navigate (
            lowest_score,
            map,
            score + 1 + cost_to_rotate_to(facing.clone(), Facing::North),
            i - 1,
            g,
            Facing::North,
            visited.clone()
        );
    }
    if i != map.len() - 1 && !visited.contains(&(i + 1, g)) && MOVEABLES.contains(&map[i + 1][g]) {
        navigate (
            lowest_score,
            map,
            score + 1 + cost_to_rotate_to(facing.clone(), Facing::South),
            i + 1,
            g,
            Facing::South,
            visited.clone()
        );
    }
    if g != 0 && !visited.contains(&(i, g - 1)) && MOVEABLES.contains(&map[i][g - 1]) {
        navigate (
            lowest_score,
            map,
            score + 1 + cost_to_rotate_to(facing.clone(), Facing::West),
            i,
            g - 1,
            Facing::West,
            visited.clone()
        );
    }
    if g != map[i].len() - 1 && !visited.contains(&(i, g + 1)) && MOVEABLES.contains(&map[i][g + 1]) {
        navigate (
            lowest_score,
            map,
            score + 1 + cost_to_rotate_to(facing.clone(), Facing::East),
            i,
            g + 1,
            Facing::East,
            visited.clone()
        );
    }
}
fn cost_to_rotate_to (
    facing: Facing,
    desired_facing: Facing
) -> usize {
    if desired_facing == facing {
        return 0;
    }

    match facing {
        Facing::North | Facing::South => {
            if desired_facing == Facing::East || desired_facing == Facing::West { return 1000; }
        },
        Facing::East | Facing::West => {
            if desired_facing == Facing::North || desired_facing == Facing::South { return 1000; }
        },
    }

    return 2000;
}
fn main() {
    let map = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut start_i = 0;
    let mut start_g = 0;

    for i in 0..map.len() {
        for g in 0..map[i].len() {
            if map[i][g] == 'S' {
                start_i = i;
                start_g = g;
            }
            print!("{}", map[i][g]);
        }
        println!("");
    }

    let mut lowest_score: Option<usize> = None;

    navigate(
        &mut lowest_score,
        &map,
        0,
        start_i,
        start_g,
        Facing::East,
        vec!()
    );

    println!("Lowest score: {lowest_score:?}");
}
