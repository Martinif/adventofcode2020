use std::fs;

#[derive(Debug)]
struct Password {
    bound_min: usize,
    bound_max: usize,
    letter: char,
    pass: String,
}

impl Password {
    fn isvalid_1(&self) -> bool{
        let count = self.pass.chars().filter(| &letter| letter == self.letter).count();
        self.bound_min <= count && count <= self.bound_max
    }

    fn isvalid_2(&self) -> bool{
        let atfirst   : bool = self.pass.chars().nth(self.bound_min-1).unwrap() == self.letter;
        let atsecond  : bool = self.pass.chars().nth(self.bound_max-1).unwrap() == self.letter;
        (atfirst | atsecond) & !(atfirst & atsecond)
    }
}

impl From<String> for Password {
    fn from(s: String) -> Password {
        let v: Vec<&str> = s.split(' ').collect();
        let bounds : Vec<&str> = v[0].split('-').collect();
        let p = Password {
            bound_min: bounds[0].parse().unwrap(),
            bound_max: bounds[1].parse().unwrap(),
            letter: v[1].chars().next().unwrap(),
            pass: v[2].to_string() };
        return p
    }
}

fn main() {
    let data = fs::read_to_string("input02.txt").expect("Unable to read file");
    let mut count_1:u32 = 0;
    let mut count_2:u32 = 0;
    for d in data.lines() {
        let p = Password::from(d.to_string());
        if p.isvalid_1() { count_1 += 1;}
    }
    for d in data.lines() {
        let p = Password::from(d.to_string());
        if p.isvalid_2() { count_2 += 1;}
    }
    println!("First task: {}", count_1);
    println!("Second task: {}", count_2);
}
