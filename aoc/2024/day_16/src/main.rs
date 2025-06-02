use std::collections::HashSet;
use std::io::Write;
const MOVEABLES: [char; 2] = ['E', '.'];

struct Node {
    val: char,
    seat: bool,
    north_record: Option<usize>,
    east_record:  Option<usize>,
    south_record: Option<usize>,
    west_record:  Option<usize>
}
#[derive(Debug, PartialEq, Clone)]
enum Facing {
    North,
    East,
    South,
    West
}
fn navigate_for_score (
    target_score: usize,
    map: &mut Vec<Vec<Node>>,
    score: usize,
    i: usize,
    g: usize,
    facing: Facing,
    mut visited: HashSet<(usize, usize)>
) {
    //println!("Navigated to ({g}, {i}) ({})", map[i][g].val);

    if map[i][g].val == 'E' {
        println!("Hit `E` with a score of {score}");

        if score == target_score {
            for (node_i, node_g) in visited.into_iter() {
                map[node_i][node_g].seat = true;
            }
        }
        
        return;
    }
    
    if score >= target_score {
        //println!("Not the path, aborting");
        return;
    }

    visited.insert((i, g));
    let up_score = score + 1 + cost_to_rotate_to(facing.clone(), Facing::North);
    if ( map[i - 1][g].north_record.is_some() && up_score == map[i - 1][g].north_record.unwrap() ) && up_score <= target_score && i != 0 && MOVEABLES.contains(&(map[i - 1][g].val)) && !visited.contains(&(i - 1, g)) {
        navigate_for_score (
            target_score,
            map,
            up_score,
            i - 1,
            g,
            Facing::North,
            visited.clone()
        );
    }
    let right_score = score + 1 + cost_to_rotate_to(facing.clone(), Facing::East);
    if map[i][g + 1].east_record.is_some() && right_score <= target_score && g != map[i].len() - 1 && MOVEABLES.contains(&(map[i][g + 1].val)) && !visited.contains(&(i, g + 1)) {
        navigate_for_score (
            target_score,
            map,
            right_score,
            i,
            g + 1,
            Facing::East,
            visited.clone()
        );
    }
    let left_score = score + 1 + cost_to_rotate_to(facing.clone(), Facing::West);
    if map[i][g - 1].west_record.is_some() && left_score <= target_score && g != 0 && MOVEABLES.contains(&(map[i][g - 1].val)) && !visited.contains(&(i, g - 1)) {
        navigate_for_score (
            target_score,
            map,
            left_score,
            i,
            g - 1,
            Facing::West,
            visited.clone()
        );
    }
    let down_score = score + 1 + cost_to_rotate_to(facing.clone(), Facing::South);
    if map[i + 1][g].south_record.is_some() && down_score <= target_score && i != map.len() - 1 && MOVEABLES.contains(&(map[i + 1][g].val)) && !visited.contains(&(i + 1, g))  {
        navigate_for_score (
            target_score,
            map,
            down_score,
            i + 1,
            g,
            Facing::South,
            visited
        );
    }

}
fn navigate (
    lowest_score: &mut Option<usize>,
    map: &mut Vec<Vec<Node>>,
    score: usize,
    i: usize,
    g: usize,
    facing: Facing
) {
    //println!("Navigated to ({g}, {i}) ({})", map[i][g].val);

    if map[i][g].val == 'E' {
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
            return;
        }
    }
    
    let expected_up_score = score + 1 + cost_to_rotate_to(facing.clone(), Facing::North);
    if i != 0 && MOVEABLES.contains(&(map[i - 1][g].val)) && 
        ( map[i - 1][g].north_record.is_none() || expected_up_score < map[i - 1][g].north_record.unwrap() ) 
    {
        map[i - 1][g].north_record = Some(expected_up_score);
        navigate (
            lowest_score,
            map,
            expected_up_score,
            i - 1,
            g,
            Facing::North
        );
    }
    let expected_right_score = score + 1 + cost_to_rotate_to(facing.clone(), Facing::East);
    if g != map[i].len() - 1 && MOVEABLES.contains(&(map[i][g + 1].val)) && 
        ( map[i][g + 1].east_record.is_none() || expected_right_score < map[i][g + 1].east_record.unwrap() )
    {
        map[i][g + 1].east_record = Some(expected_right_score);
        navigate (
            lowest_score,
            map,
            expected_right_score,
            i,
            g + 1,
            Facing::East
        );
    }
    let expected_left_score = score + 1 + cost_to_rotate_to(facing.clone(), Facing::West);
    if g != 0 && MOVEABLES.contains(&(map[i][g - 1].val)) && 
        ( map[i][g - 1].west_record.is_none() || expected_left_score < map[i][g - 1].west_record.unwrap() ) 
    {
        map[i][g - 1].west_record = Some(score);
        navigate (
            lowest_score,
            map,
            expected_left_score,
            i,
            g - 1,
            Facing::West
        );
    }
    let expected_down_score = score + 1 + cost_to_rotate_to(facing.clone(), Facing::South);
    if i != map.len() - 1 && MOVEABLES.contains(&(map[i + 1][g].val)) && 
        ( map[i + 1][g].south_record.is_none() || expected_down_score < map[i + 1][g].south_record.unwrap() ) 
    {
        map[i + 1][g].south_record = Some(expected_down_score);
        navigate (
            lowest_score,
            map,
            expected_down_score,
            i + 1,
            g,
            Facing::South
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
    let mut map = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| Node {
                    val: ch,
                    seat: false,
                    north_record: None,
                    east_record:  None,
                    south_record: None,
                    west_record:  None
                })
                .collect::<Vec<Node>>()
        })
        .collect::<Vec<Vec<Node>>>();

    let mut start_i = 0;
    let mut start_g = 0;

    for i in 0..map.len() {
        for g in 0..map[i].len() {
            if map[i][g].val == 'S' {
                start_i = i;
                start_g = g;
            }
            print!("{}", map[i][g].val);
        }
        println!("");
    }

    let mut lowest_score: Option<usize> = None;

    navigate(
        &mut lowest_score,
        &mut map,
        0,
        start_i,
        start_g,
        Facing::East
    );

    if let Some(lowest_score) = lowest_score {
        println!("Lowest score: {lowest_score:?}");

        navigate_for_score(
            lowest_score, 
            &mut map,
            0,
            start_i,
            start_g,
            Facing::East,
            HashSet::new()
        );
    } else {
        println!("No valid path!");

        return;
    }

    let mut tiles = 1usize;
    for i in 0..map.len() {
        for g in 0..map[i].len() {
            if map[i][g].seat {
                print!("O");
                tiles += 1;

                continue;
            }
            print!("{}", map[i][g].val);
        }
        println!("");
    }

    println!("There are {tiles} tiles ^^");

    let mut file = std::fs::File::create("output.txt").unwrap();

    for i in 0..map.len() {
        for g in 0..map[i].len() {
            write!(file, " {}|{:05}-{:05}-{:05}-{:05}", 
                map[i][g].val, 
                map[i][g].north_record.unwrap_or(0),
                map[i][g].east_record.unwrap_or(0),
                map[i][g].south_record.unwrap_or(0),
                map[i][g].west_record.unwrap_or(0),
            ).unwrap();
        }
        write!(file, "\n").unwrap();
    }
}
