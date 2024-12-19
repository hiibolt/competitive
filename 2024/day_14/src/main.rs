use regex::Regex;
use std::io::Write;

struct Map {
    robots: Vec<Robot>,
    x: usize,
    y: usize
}
impl Map {
    fn render ( &self, file: &mut std::fs::File, seconds: i32  ) {
        let mut map = vec!(vec!(0; self.x); self.y);
        for robot in &self.robots {
            map[robot.y as usize][robot.x as usize] += 1;
        }


        let mut stringified = String::new();
        let mut had_long = false;
        for row in &map {
            for g in 0..row.len() {
                let mut horizontal = 0;
                for g_cursor in (g + 1)..row.len() {
                    if row[g_cursor] > 0 {
                        horizontal += 1;
                    } else {
                        break;
                    }
                }
                if horizontal > 5 {
                    had_long = true;
                    println!(":3");
                }

                if row[g] == 0 {
                    stringified += ".";
                    continue;
                }

                stringified += &format!("{}", row[g]);
            }
            stringified += "\n";
        }
        stringified += &format!("\nSeconds ellapsed: {seconds}");
        if had_long {
            write!(file, "{}", &stringified).unwrap();
        }
    }
    fn pass_second ( &mut self ) {
        for robot in &mut self.robots {
            robot.pass_second( self.x, self.y );
        }
    }
    fn _safety_factor ( self ) -> usize {
        let quadrant_1 = self.robots
            .iter()
            .filter(|rb| ((*rb).x as usize) < self.x / 2 && ((*rb).y as usize) < self.y / 2 )
            .count();
        let quadrant_2 = self.robots
            .iter()
            .filter(|rb| ((*rb).x as usize) > self.x / 2 && ((*rb).y as usize) < self.y / 2 )
            .count();
        let quadrant_3 = self.robots
            .iter()
            .filter(|rb| ((*rb).x as usize) < self.x / 2 && ((*rb).y as usize) > self.y / 2 )
            .count();
        let quadrant_4 = self.robots
            .iter()
            .filter(|rb| ((*rb).x as usize) > self.x / 2 && ((*rb).y as usize) > self.y / 2 )
            .count();

        quadrant_1 * quadrant_2 * quadrant_3 * quadrant_4
    }
}
struct Robot {
    x: i32,
    y: i32,
    v_x: i32,
    v_y: i32
}
impl Robot {
    fn pass_second ( &mut self, board_x: usize, board_y: usize ) {
        self.x = (self.x + self.v_x).rem_euclid(board_x as i32);
        self.y = (self.y + self.v_y).rem_euclid(board_y as i32);
    }
}

fn main() {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)")
        .unwrap();
    let data = std::fs::read_to_string("input.txt")
        .unwrap();

    let robots = re.captures_iter(&data)
        .map(|c| c.extract())
        .map(|(_, [x, y, v_x, v_y])| {
            let x = x.parse::<i32>().unwrap();
            let y = y.parse::<i32>().unwrap();
            let v_x = v_x.parse::<i32>().unwrap();
            let v_y = v_y.parse::<i32>().unwrap();

            Robot { x, y, v_x, v_y }
        })
        .collect::<Vec<Robot>>();
    let mut map = Map { robots, x: 101, y: 103 };
    let mut seconds_ellapsed = 0;

    let mut file = std::fs::File::create("output.txt")
        .unwrap();
    map.render(&mut file, seconds_ellapsed);

    let seconds = 100000;
    for _ in 1..=seconds {
        seconds_ellapsed += 1;
        map.pass_second();
        map.render(&mut file, seconds_ellapsed);
    }

    /*
    println!("\nAfter {seconds} seconds...");
    map.render();

    let safety_factor = map.safety_factor();
    println!("Safety factor: {safety_factor}");
     */
}
