use std::{fs};
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref AZ : String = buildaz();
}

fn buildaz() -> String{
    let mut az = "".to_string();
    let mut c = 'a';
    az.push(c);
    for _ in 1..26 {
        c = std::char::from_u32(c as u32 + 1).unwrap_or(c);
        az.push(c);
    }
    az
}

fn atleast1occ(customs : &str) -> u32{
    let mut counter : u32 = 0;
    for c in AZ.chars() {
        let v: Vec<char> = customs.chars().collect();
        if v.contains(&c) {
            counter += 1;
        }
    }
    counter;
}

fn occinall(customs : &str) -> u32{
    let mut counter : u32 = 0;
    let persons : Vec<&str> = customs.split('\n').collect();
    for c in AZ.chars() {
        let mut occurs = true;
        for person in &persons {
            if !(person.contains(c)) & !(person ==&"") {
                occurs = false;
            }
        }
        if occurs { counter += 1;}
    }
   counter;
}


fn main() {
    let data = fs::read_to_string("input06.txt").expect("Unable to read file");
    let mut counter_1: u32 = 0; // passports that fulfill criteria from task 1
    let mut counter_2: u32 = 0; // passports that fulfill criteria from task 2
    for group in data.split("\n\n"){
        counter_1 += atleast1occ(group);
        counter_2 += occinall(group);
    }
    println!("First task {}", counter_1);
    println!("Second task {}", counter_2);
}
