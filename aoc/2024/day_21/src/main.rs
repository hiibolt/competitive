use std::collections::{HashSet, VecDeque};
use memoize::memoize;

const KEYPAD: [[char; 3]; 4] = [
    ['7','8','9'],
    ['4','5','6'],
    ['1','2','3'],
    [' ','0','A']
];
const DIRPAD: [[char; 3]; 2] = [
    [' ','^','A'],
    ['<','v','>']
];
#[memoize]
fn find_keypad ( 
    to_find: char
) -> Option<(i32, i32)> {
    for i in 0..KEYPAD.len() {
        for g in 0..KEYPAD[i].len() {
            if KEYPAD[i][g] == to_find {
                return Some((i as i32, g as i32));
            }
        }
    }
    None
}
#[memoize]
fn find_dirpad ( 
    to_find: char
) -> Option<(i32, i32)> {
    for i in 0..DIRPAD.len() {
        for g in 0..DIRPAD[i].len() {
            if DIRPAD[i][g] == to_find {
                return Some((i as i32, g as i32));
            }
        }
    }
    None
}
#[memoize]
fn filter_illegal_dirpad (
    start: (i32, i32),
    paths: Vec<String>
) -> Vec<String> {
    let start = (start.0 as usize, start.1 as usize);

    let mut ret = Vec::new();
    'outer: for path in paths {
        let mut i = start.0;
        let mut g = start.1;

        for dir in path.chars() {
            match dir {
                '^' => i -= 1,
                'v' => i += 1,
                '>' => g += 1,
                '<' => g -= 1,
                'A' => (),
                _ => panic!("Illegal direction character!")
            }
            if DIRPAD[i][g] == ' ' {
                continue 'outer;
            }
        }

        ret.push(path);
    }

    ret
}
#[memoize]
fn filter_illegal_keypad (
    start: (i32, i32),
    _target: (i32, i32),
    paths: Vec<String>
) -> Vec<String> {
    let start = (start.0 as usize, start.1 as usize);

    let mut ret = Vec::new();
    'outer: for path in paths {
        let mut i = start.0;
        let mut g = start.1;

        for dir in path.chars() {
            match dir {
                '^' => i -= 1,
                'v' => i += 1,
                '>' => g += 1,
                '<' => g -= 1,
                'A' => (),
                _ => panic!("Illegal direction character!")
            }
            if KEYPAD[i][g] == ' ' {
                continue 'outer;
            }
        }

        ret.push(path);
    }

    ret
}
fn possible_keycode_moves (
    current_char: char,
    mut remaining_chars: VecDeque<char>
) -> HashSet<String> {
    if remaining_chars.is_empty() {
        return HashSet::new();
    }

    let next = remaining_chars.pop_front().unwrap();

    let cur_coords = find_keypad(current_char).unwrap();
    let target_coords = find_keypad(next).unwrap();

    /*
    println!(
        "Current Coords: ({}, {})\nTarget Coords: ({}, {})",
        cur_coords.0,
        cur_coords.1,
        target_coords.0,
        target_coords.1
    ); */

    let diff = ((cur_coords.0 - target_coords.0), (target_coords.1 - cur_coords.1));
    //println!("Diff: {diff:?}");
    let initial = match diff {
        ( 0,  0) => vec!(String::from("A")),

        // Straight Lines
        // 1-length
        ( 0,  1) => vec!(String::from(">A")),
        ( 0, -1) => vec!(String::from("<A")),
        ( 1,  0) => vec!(String::from("^A")),
        (-1,  0) => vec!(String::from("vA")),
        // 2-length
        ( 0,  2) => vec!(String::from(">>A")),
        ( 0, -2) => vec!(String::from("<<A")),
        ( 2,  0) => vec!(String::from("^^A")),
        (-2,  0) => vec!(String::from("vvA")),
        // 3-length (only vertical)
        ( 3,  0) => vec!(String::from("^^^A")),
        (-3,  0) => vec!(String::from("vvvA")),

        // Squares
        // 1x1
        ( 1,  1) => filter_illegal_keypad(cur_coords, target_coords, vec!(String::from(">^A"), String::from("^>A"))),
        ( 1, -1) => filter_illegal_keypad(cur_coords, target_coords, vec!(String::from("^<A"), String::from("<^A"))),
        (-1,  1) => filter_illegal_keypad(cur_coords, target_coords, vec!(String::from("v>A"), String::from(">^A"))),
        (-1, -1) => filter_illegal_keypad(cur_coords, target_coords, vec!(String::from("v<A"), String::from("<vA"))),
        // 2x2
        ( 2,  2) => filter_illegal_keypad(cur_coords, target_coords, vec!(
            String::from("^^>>A"), String::from(">>^^A"),
            //String::from(">^>^A"), String::from("^>^>A"),
            //String::from(">^^>A"), String::from("^>>^A"),
        )),
        (-2,  2) => filter_illegal_keypad(cur_coords, target_coords, vec!(
            String::from("vv>>A"), String::from(">>vvA"),
            //String::from(">v>vA"), String::from("v>v>A"),
            //String::from(">vv>A"), String::from("v>>vA"),
        )),
        ( 2, -2) => filter_illegal_keypad(cur_coords, target_coords, vec!(
            String::from("^^<<A"), String::from("<<^^A"),
            //String::from("<^<^A"), String::from("^<^<A"),
            //String::from("<^^<A"), String::from("^<<^A"),
        )),
        (-2, -2) => filter_illegal_keypad(cur_coords, target_coords, vec!(
            String::from("vv<<A"), String::from("<<vvA"),
            //String::from("<v<vA"), String::from("v<v<A"),
            //String::from("<vv<A"), String::from("v<<vA"),
        )),

        // Rectangles
        // 1x2 and 2x1
        ( 1,  2) => filter_illegal_keypad(cur_coords, target_coords, vec!(
            String::from(">>^A"), String::from("^>>A"), //String::from(">^>A")
        )),
        ( 1, -2) => filter_illegal_keypad(cur_coords, target_coords, vec!(
            String::from("<<^A"), String::from("^<<A"), //String::from("<^<A")
        )),
        (-1,  2) => filter_illegal_keypad(cur_coords, target_coords, vec!(
            String::from(">>vA"), String::from("v>>A"), //String::from(">v>A")
        )),
        (-1, -2) => filter_illegal_keypad(cur_coords, target_coords, vec!(
            String::from("<<vA"), String::from("v<<A"), //String::from("<v<A")
        )),
        ( 2,  1) => filter_illegal_keypad(cur_coords, target_coords, vec!(
            String::from("^^>A"), String::from(">^^A"), //String::from("^>^A")
        )),
        (-2,  1) => filter_illegal_keypad(cur_coords, target_coords, vec!(
            String::from("vv>A"), String::from(">vvA"), //String::from("v>vA")
        )),
        ( 2, -1) => filter_illegal_keypad(cur_coords, target_coords, vec!(
            String::from("^^<A"), String::from("<^^A"), //String::from("^<^A")
        )),
        (-2, -1) => filter_illegal_keypad(cur_coords, target_coords, vec!(
            String::from("vv<A"), String::from("<vvA"), //String::from("v<vA")
        )),
        // 1x3
        ( 3,  1) => filter_illegal_keypad(cur_coords, target_coords, vec!(
            String::from(">^^^A"), String::from("^^^>A"),
            //String::from("^^>^A"), String::from("^>^^A"),
        )),
        (-3,  1) => filter_illegal_keypad(cur_coords, target_coords, vec!(
            String::from(">vvvA"), String::from("vvv>A"),
            //String::from("vv>vA"), String::from("v>vvA"),
        )),
        ( 3, -1) => filter_illegal_keypad(cur_coords, target_coords, vec!(
            String::from("<^^^A"), String::from("^^^>A"),
            //String::from("^^<^A"), String::from("^<^^A"),
        )),
        (-3, -1) => filter_illegal_keypad(cur_coords, target_coords, vec!(
            String::from("<vvvA"), String::from("vvv>A"),
            //String::from("vv<vA"), String::from("v<vvA"),
        )),
        _ => panic!("Unhandled scenario!")
    };

    let mut final_ret: HashSet<String> = HashSet::new();
    let to_append = possible_keycode_moves(
        next,
        remaining_chars.clone()
    );
    for ele in &initial {
        if to_append.is_empty() {
            final_ret.insert(ele.clone());
            continue;
        }
        for to_append in &to_append {
            final_ret.insert(ele.clone() + to_append);
        }
    }

    final_ret
}
#[memoize]
fn moves_to_get_to_dircode (
    cur:  char,
    next: char,
) -> Vec<String> {
    match (cur, next) {
        ('^', '^') => vec!(String::from("A")),
        ('^', 'A') => vec!(String::from(">A")),
        ('^', 'v') => vec!(String::from("vA")),
        ('^', '<') => vec!(String::from("v<A")),
        ('^', '>') => vec!(String::from("v>A"), String::from(">vA")),

        ('A', 'A') => vec!(String::from("A")),
        ('A', '^') => vec!(String::from("<A")),
        ('A', '>') => vec!(String::from("vA")),
        ('A', 'v') => vec!(String::from("<vA"), String::from("v<A")),
        ('A', '<') => vec!(String::from("v<<A")),

        ('>', '>') => vec!(String::from("A")),
        ('>', 'A') => vec!(String::from("^A")),
        ('>', '^') => vec!(String::from("<^A"), String::from("^<A")),
        ('>', 'v') => vec!(String::from("<A")),
        ('>', '<') => vec!(String::from("<<A")),

        ('v', 'v') => vec!(String::from("A")),
        ('v', '<') => vec!(String::from("<A")),
        ('v', '>') => vec!(String::from(">A")),
        ('v', '^') => vec!(String::from("^A")),
        ('v', 'A') => vec!(String::from("^>A"), String::from(">^A")),

        ('<', '<') => vec!(String::from("A")),
        ('<', 'v') => vec!(String::from(">A")),
        ('<', '>') => vec!(String::from(">>A")),
        ('<', '^') => vec!(String::from(">^A")),
        ('<', 'A') => vec!(String::from(">>^A")),

        _ => panic!("Unhandled scenario!")
    }
}
#[memoize]
fn min_length (
    st: String,
    depth: usize
) -> usize {
    if depth > 24 {
        return st.len();
    }

    let mut ret = 0;

    let mut current = 'A';
    for target in st.chars() {
        let mut lowest_option: Option<usize> = None;

        for possible_string in moves_to_get_to_dircode( current, target ) {
            let minimum_len = min_length(possible_string, depth + 1);
            if let Some(lowest) = lowest_option {
                lowest_option = Some(lowest.min(minimum_len));
            } else {
                lowest_option = Some(minimum_len);
            }
        }
        
        ret += lowest_option.unwrap();
        current = target;
    }

    ret
}
fn main() {
    let keycodes = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|st| st.chars().collect::<VecDeque<char>>())
        .collect::<Vec<VecDeque<char>>>();

    let mut total_complexity = 0usize;
    for keycode in keycodes {
        println!("{keycode:?}");
        let possible_directions = possible_keycode_moves(
            'A', 
            keycode.clone()
        );
        let keycode_value = keycode.iter()
            .filter(|&ch| ch.is_numeric())
            .collect::<String>()
            .parse::<usize>().unwrap();

        let mut lowest: Option<usize> = None;
        for possible in possible_directions {
            println!("{possible:?}");

            let minimum_length = min_length(possible, 0);

            if let Some(lowest_val) = lowest {
                lowest = Some(lowest_val.min(minimum_length));
            } else {
                lowest = Some(minimum_length);
            }
        }
        if let Some(lowest) = lowest {
            total_complexity += lowest * keycode_value;
        } else {
            panic!("Didn't have a valid solution!");
        }
    }

    println!("Total complexity: {total_complexity}");
}
