enum Node {
    Wall,
    Box,
    Free
}
impl Node {
    fn display ( &self ) {
        match self {
            Node::Wall => print!("#"),
            Node::Box => print!("O"),
            Node::Free => print!(" ")
        }
    }
}

struct Robot {
    i: usize,
    g: usize
}
impl Robot {
    fn mv (
        &mut self,
        board: &mut Vec<Vec<Node>>,
        mv: char
    ) {
        println!("Moving {mv}");
        match mv {
            '^' => {
                if self.i == 0 {
                    return;
                }

                match board[self.i - 1][self.g] {
                    Node::Wall => { },
                    Node::Free => {
                        self.i -= 1;
                    },
                    Node::Box => {
                        for i in 0..(self.i - 2) {
                            let i = self.i - i - 2;

                            if let Node::Free = board[i][self.g] {
                                board[i][self.g] = Node::Box;
                                board[self.i - 1][self.g] = Node::Free;
                                self.i -= 1;
                                break;
                            }
                            if let Node::Wall = board[i][self.g] {
                                break;
                            }
                        }
                    }
                }
            },
            'v' => {
                if self.i == board.len() - 1 {
                    return;
                }

                match board[self.i + 1][self.g] {
                    Node::Wall => { },
                    Node::Free => {
                        self.i += 1;
                    },
                    Node::Box => {
                        for i in (self.i + 2)..(board.len() - 1) {
                            if let Node::Free = board[i][self.g] {
                                board[i][self.g] = Node::Box;
                                board[self.i + 1][self.g] = Node::Free;
                                self.i += 1;
                                break;
                            }
                            if let Node::Wall = board[i][self.g] {
                                break;
                            }
                        }
                    }
                }
            },
            '<' => {
                if self.g == 0 {
                    return;
                }

                match board[self.i][self.g - 1] {
                    Node::Wall => { },
                    Node::Free => {
                        self.g -= 1;
                    },
                    Node::Box => {
                        for g in 0..(self.g - 2) {
                            let g = self.g - g - 2;

                            if let Node::Free = board[self.i][g] {
                                board[self.i][g] = Node::Box;
                                board[self.i][self.g - 1] = Node::Free;
                                self.g -= 1;
                                break;
                            }
                            if let Node::Wall = board[self.i][g] {
                                break;
                            }
                        }
                    }
                }
            },
            '>' => {
                if self.g == board[0].len() - 1 {
                    return;
                }

                match board[self.i][self.g + 1] {
                    Node::Wall => { },
                    Node::Free => {
                        self.g += 1;
                    },
                    Node::Box => {
                        for g in (self.g + 2)..(board[0].len() - 1) {
                            if let Node::Free = board[self.i][g] {
                                board[self.i][g] = Node::Box;
                                board[self.i][self.g + 1] = Node::Free;
                                self.g += 1;
                                break;
                            }
                            if let Node::Wall = board[self.i][g] {
                                break;
                            }
                        }
                    }
                }
            },
            _ => {}
        }
    }
    fn render (
        &self,
        board: &Vec<Vec<Node>>
    ) {
        for i in 0..board.len() {
            for g in 0..board[i].len() {
                if i == self.i && g == self.g {
                    print!("@");
                    continue;
                }

                board[i][g].display();
            }
            println!();
        }
    }
}


fn main() {
    let data = std::fs::read_to_string("input.txt")
        .unwrap();

    let mut robot: Option<Robot> = None;
    let moves = data.split("\n\n")
        .nth(1).unwrap()
        .lines()
        .flat_map(|ln| ln.chars().collect::<Vec<char>>() )
        .collect::<Vec<char>>();
    let mut map = data.split("\n\n")
        .next().unwrap()
        .lines()
        .enumerate()
        .map(|(i, ln)| {
            ln.chars()
                .enumerate()
                .map(|(g, ch)| {
                    match ch {
                        '#' => Node::Wall,
                        'O' => Node::Box,
                        '.' => Node::Free,
                        '@' => {
                            robot = Some(Robot {
                                i,
                                g
                            });
                            Node::Free
                        },
                        _  => Node::Free
                    }
                })
                .collect::<Vec<Node>>()
        })
        .collect::<Vec<Vec<Node>>>();

    let mut robot = robot.unwrap();
    

    robot.render(
        &map
    );

    for &mv in &moves {
        robot.mv( 
            &mut map,
            mv
        );

        //robot.render(
        //    &map
        //);
    }

    let mut sum = 0usize;
    for (i, row) in map.iter().enumerate() {
        for (g, node) in row.iter().enumerate() {
            if let Node::Box = node {
                sum += 100 * i + g;
            }
        }
    }
    println!("GPS coordinate sum: {sum}");
}
