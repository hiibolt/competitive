use std::collections::HashSet;
use std::sync::{ Arc, Mutex };

#[derive(Clone)]
struct Cheat {
    in_i: usize,
    in_g: usize,
    out_i: usize,
    out_g: usize
}
#[derive(Clone)]
struct Node {
    val: char,
    records: Vec<Option<usize>>
}
struct Board {
    width: usize,
    height: usize,
    board: Vec<Vec<Node>>,
    
    start_i: usize,
    start_g: usize,
    end_i:   usize,
    end_g:   usize,
    lowest_score: Option<usize>,
    lowest_visited: Option<Vec<(usize, usize)>>
}
impl Board {
    fn new (
        board: Vec<Vec<Node>>,
        start_i: usize,
        start_g: usize,
        end_i: usize,
        end_g: usize
    ) -> Self {
        Self {
            width: board[0].len(),
            height: board.len(),
            board,

            start_i,
            start_g,
            end_i,
            end_g,
            lowest_score: None,
            lowest_visited: None
        }
    }
    fn render ( &self ) {
        println!("Current board state:");
        for row in self.board.iter() {
            for node in row.iter() {
                if node.val == '#' {
                    print!("####");
                    continue;
                }
                print!("    ");
            }
            println!();
            for node in row.iter() {
                if node.val == '#' {
                    print!("####");
                    continue;
                }
                print!(
                    " {:02} ", 
                    node.records[0]
                        .and_then(|rc| Some(rc.to_string()))
                        .unwrap_or(String::from(":3"))
                );
            }
            println!();
            for node in row.iter() {
                if node.val == '#' {
                    print!("####");
                    continue;
                }
                print!(
                    " {:02} ", 
                    node.records[1]
                        .and_then(|rc| Some(rc.to_string()))
                        .unwrap_or(String::from(":3"))
                );
            }
            println!();
            for node in row.iter() {
                if node.val == '#' {
                    print!("####");
                    continue;
                }
                print!("    ");
            }
            println!();
        }
    }
    fn reset (
        &mut self
    ) {
        for i in 0..self.board.len() {
            for g in 0..self.board[i].len() {
                self.board[i][g].records = vec!(None, None);
            }
        }
        self.lowest_score = None;
    }
    fn shortest_normal_distance (
        &mut self,
        cheat: Option<Cheat>
    ) -> Option<usize> {
        self.traverse(
            self.start_i,
            self.start_g,
            0,

            cheat
        );
        
        self.lowest_score
    }
    fn traverse ( 
        &mut self,
        i: usize,
        g: usize,
        score: usize,

        cheat: Option<Cheat>
    ) {
        //println!("Traversed to ({}, {}) with a score of {}", g, i, score);

        if (i, g) == (self.end_i, self.end_g) {
            //println!("Hit the final node with a score of {score}!");

            if let Some(lowest_score) = self.lowest_score {
                if score < lowest_score {
                    self.lowest_score = Some(score);
                }
            } else {
                self.lowest_score = Some(score);
            }
            return;
        }

        if i > 0 && self.board[i - 1][g].val == '.' &&
            ( self.board[i - 1][g].records[0].is_none() || 
              (score + 1) < self.board[i - 1][g].records[0].unwrap() )
        {
            self.board[i - 1][g].records[0] = Some(score + 1);
            self.traverse (
                i - 1,
                g,
                score + 1,
                cheat.clone()
            );
        }
        if i < self.height - 1 && self.board[i + 1][g].val == '.' &&
            ( self.board[i + 1][g].records[0].is_none() || 
              (score + 1) < self.board[i + 1][g].records[0].unwrap() )
        {
            self.board[i + 1][g].records[0] = Some(score + 1);
            self.traverse (
                i + 1,
                g,
                score + 1,
                cheat.clone()
            );
        }
        if g > 0 && self.board[i][g - 1].val == '.' &&
            ( self.board[i][g - 1].records[0].is_none() || 
              (score + 1) < self.board[i][g - 1].records[0].unwrap() )
        {
            self.board[i][g - 1].records[0] = Some(score + 1);
            self.traverse (
                i,
                g - 1,
                score + 1,
                cheat.clone()
            );
        }
        if g < self.width - 1 && self.board[i][g + 1].val == '.' &&
            ( self.board[i][g + 1].records[0].is_none() ||
            (score + 1) < self.board[i][g + 1].records[0].unwrap() )
        {
            self.board[i][g + 1].records[0] = Some(score + 1);
            self.traverse (
                i,
                g + 1,
                score + 1,
                cheat
            );
        }

    }
}
fn taxicab_distance (
    i_1: usize,
    g_1: usize,
    i_2: usize,
    g_2: usize
) -> usize {
    i_1.abs_diff(i_2) + g_1.abs_diff(g_2)
}
fn main() {
    let mut start_i: Option<usize> = None;
    let mut start_g: Option<usize> = None;
    let mut end_i:   Option<usize> = None;
    let mut end_g:   Option<usize> = None;
    let board = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i, ln)| {
            ln.chars()
                .enumerate()
                .map(|(g, ch)| {
                    match ch {
                        'S' => {
                            start_i = Some(i);
                            start_g = Some(g);
                            Node { val: '.', records: vec!(None, None) }
                        },
                        'E' => {
                            end_i = Some(i);
                            end_g = Some(g);
                            Node { val: '.', records: vec!(None, None) }
                        },
                        '.' => Node { val: '.', records: vec!(None, None) },
                        '#' => Node { val: '#', records: vec!(None, None) },
                        _ => panic!("Invalid input!")
                    }
                })
                .collect::<Vec<Node>>()
        })
        .collect::<Vec<Vec<Node>>>();

    let start_i = start_i.unwrap();
    let start_g = start_g.unwrap();
    let end_i   = end_i.unwrap();
    let end_g   = end_g.unwrap();
    let mut base_board = Board::new(
        board.clone(),
        start_i,
        start_g,
        end_i,
        end_g
    );
    println!(":3");

    let lowest_score = base_board.shortest_normal_distance(None).unwrap();
    println!(":3");
    base_board.render();

    println!("Total cheats >100: {}", 0);
}
