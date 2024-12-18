use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
enum Facing {
    North,
    East,
    South,
    West
}
#[derive(Debug, Clone)]
struct NodeToProcess {
    x: usize,
    y: usize,
    score: usize,
    facing: Facing
}
impl PartialEq<NodeToProcess> for NodeToProcess {
    fn eq(&self, other: &NodeToProcess) -> bool {
        self.x == other.x && self.y == other.y && self.facing == other.facing
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
    let board = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut start_x = 0;
    let mut start_y = 0;

    for i in 0..board.len() {
        for g in 0..board[i].len() {
            if board[i][g] == 'S' {
                start_x = g;
                start_y = i;
            }
            print!("{}", board[i][g]);
        }
        println!("");
    }

    let mut visited: Vec<NodeToProcess> = Vec::new();
    let mut to_process: VecDeque<NodeToProcess> = VecDeque::new();
    to_process.push_back(NodeToProcess {
        x: start_x,
        y: start_y,
        score: 0,
        facing: Facing::East
    });
    let mut lowest_score: Option<usize> = None;
    while let Some(node) = to_process.pop_front() {
        visited.push(node.clone());

        println!("{:#?}", node);
        
        if board[node.y][node.x] == 'E' {
            if let Some(current_score) = lowest_score {
                lowest_score = Some(current_score.min(node.score));
            } else {
                lowest_score = Some(node.score);
            }
        }
        
        if node.y > 0 && ( board[node.y - 1][node.x] == '.' || board[node.y - 1][node.x] == 'E' ) {
            let would_add = NodeToProcess {
                x: node.x,
                y: node.y - 1,
                score: node.score + cost_to_rotate_to(node.facing.clone(), Facing::North) + 1,
                facing: Facing::North
            };
            if !visited.contains(&would_add) {
                to_process.push_back(would_add);
            }
            
        }
        if node.x < board[0].len() - 1 && ( board[node.y][node.x + 1] == '.' || board[node.y][node.x + 1] == 'E' ) {
            let would_add = NodeToProcess {
                x: node.x + 1,
                y: node.y,
                score: node.score + cost_to_rotate_to(node.facing.clone(), Facing::East) + 1,
                facing: Facing::East
            };
            if !visited.contains(&would_add) {
                to_process.push_back(would_add);
            }
        }
        if node.y < board.len() - 1 && ( board[node.y + 1][node.x] == '.' || board[node.y + 1][node.x] == 'E' ) {
            let would_add = NodeToProcess {
                x: node.x,
                y: node.y + 1,
                score: node.score + cost_to_rotate_to(node.facing.clone(), Facing::South) + 1,
                facing: Facing::South
            };
            if !visited.contains(&would_add) {
                to_process.push_back(would_add);
            }
        }
        if node.x > 0 && ( board[node.y][node.x - 1] == '.' || board[node.y][node.x - 1] == 'E' ) {
            let would_add = NodeToProcess {
                x: node.x - 1,
                y: node.y,
                score: node.score + cost_to_rotate_to(node.facing.clone(), Facing::West) + 1,
                facing: Facing::West
            };
            if !visited.contains(&would_add) {
                to_process.push_back(would_add);
            }
        }
    }

    println!("Lowest possible score: {lowest_score:?}");
}
