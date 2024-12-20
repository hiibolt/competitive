use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Byte {
    i: usize,
    g: usize
}

#[derive(Clone, PartialEq)]
struct BoardNode {
    val: char,
    record: Option<usize>
}
struct Board {
    width:         usize,
    height:        usize,
    bytes:         VecDeque<Byte>,
    board:         Vec<Vec<BoardNode>>,

    highest_score: Option<usize>
}
impl Board {
    fn new (
        i: usize,
        g: usize,
        bytes: VecDeque<Byte>
    ) -> Self {
        Self {
            width: g,
            height: i,
            bytes,
            board: vec!(vec!(BoardNode { val: '.', record: None }; g); i),

            highest_score: None
        }
    }
    fn render ( &self ) {
        println!("Current board:");
        for (i, row) in self.board.iter().enumerate() {
            for (g, node) in row.iter().enumerate() {
                print!("{} {:02} ", self.board[i][g].val, self.board[i][g].record.unwrap_or(0));
            }
            println!();
        }
    }
    fn drop ( &mut self, count: usize ) {
        for _ in 0..count {
            if let Some(Byte { i, g}) = self.bytes.pop_front() {
                self.board[i][g].val = '#';
            }
        }
    }
    fn reset ( &mut self ) {
        self.highest_score = None;

        for i in 0..self.height {
            for g in 0..self.width {
                self.board[i][g].record = None;
            }
        }
    }
    fn traverse ( 
        &mut self,
        i: usize,
        g: usize,
        score: usize
    ) -> Option<usize> {
        //println!("Traversed to ({g}, {i})");

        if (i, g) == (self.height - 1, self.width - 1) {
            //println!("Hit the final node with a score of {score}!");

            if let Some(highest_score) = self.highest_score {
                if score < highest_score {
                    self.highest_score = Some(score);
                }
            } else {
                self.highest_score = Some(score);
            }
        }

        if i > 0 && self.board[i - 1][g].val == '.' &&
            ( self.board[i - 1][g].record.is_none() || (score + 1) < self.board[i - 1][g].record.unwrap() )
        {
            self.board[i - 1][g].record = Some(score + 1);
            self.traverse (
                i - 1,
                g,
                score + 1
            );
        }
        if i < self.height - 1 && self.board[i + 1][g].val == '.' &&
            ( self.board[i + 1][g].record.is_none() || (score + 1) < self.board[i + 1][g].record.unwrap() )
        {
            self.board[i + 1][g].record = Some(score + 1);
            self.traverse (
                i + 1,
                g,
                score + 1
            );
        }
        if g > 0 && self.board[i][g - 1].val == '.' &&
            ( self.board[i][g - 1].record.is_none() || (score + 1) < self.board[i][g - 1].record.unwrap() )
        {
            self.board[i][g - 1].record = Some(score + 1);
            self.traverse (
                i,
                g - 1,
                score + 1
            );
        }
        if g < self.width - 1 && self.board[i][g + 1].val == '.' &&
            ( self.board[i][g + 1].record.is_none() || (score + 1) < self.board[i][g + 1].record.unwrap() )
        {
            self.board[i][g + 1].record = Some(score + 1);
            self.traverse (
                i,
                g + 1,
                score + 1
            );
        }

        self.highest_score
    }
}

fn main() {
    let bytes = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|ln| Byte{
            i: ln.split(",").nth(1).unwrap().parse::<usize>().unwrap(),
            g: ln.split(",").next().unwrap().parse::<usize>().unwrap()
        })
        .collect::<VecDeque<Byte>>();

    println!("{bytes:?}");

    let mut board = Board::new(71, 71, bytes.clone());
    board.render();

    for i in 1..bytes.len() {
        println!("Drop #{i} ({}, {})", bytes[i - 1].g, bytes[i - 1].i);
        board.drop(1);
        board.reset();

        let lowest_score = board.traverse(0, 0, 0);
        println!("Score: {lowest_score:?}");

        if lowest_score.is_none() {
            println!("First bad byte after {i} drops");
            println!("Coordinates: ({},{})", bytes[i - 1].g, bytes[i - 1].i);
            return;
        }
    }
}
