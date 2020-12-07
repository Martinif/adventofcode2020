use std::{fs};

struct BoardingPass {
    row : i32,
    col : i32
}

impl BoardingPass {
    fn get_id(&self) -> i32 {
        self.row * 8 + self.col
    }
}

impl From<String> for BoardingPass {
    fn from(s: String) -> BoardingPass {
        let mut range = 128;
        let mut offset = 0;
        let mut p = BoardingPass{
            row: 0,
            col: 0
        };
        for c in s[..7].chars() {
            match c {
                'F' => {
                    range = range/2;
                }
                'B' => {
                    range = range/2;
                    offset = offset+range;
                }
                _ => println!("FAIL")
            }
        }
        p.row = offset;
        range = 8;
        offset = 0;
        for c in s[7..].chars() {
            match c {
                'L' => {
                    range = range/2;
                }
                'R' => {
                    range = range/2;
                    offset = offset+range;
                }
                _ => println!("ERROR")
            }
        }
        p.col = offset;
        p
    }
}


fn main() {
    let data = fs::read_to_string("input/input05.txt").expect("Unable to read file");
    let mut ids : Vec<i32> = data.lines().map(|bps| BoardingPass::from(bps.to_string()).get_id()).collect();
    ids.sort();
    println!("Task 1: {}", ids.last().unwrap());

    for bp in ids.iter().skip(1) {
        if !(ids.contains(&(bp+1))) & (&(bp+1) <= ids.last().unwrap()) {
            println!("Task 2: {}", bp+1)
        }
    }

}
