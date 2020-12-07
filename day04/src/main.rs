#[doc(inline)]
#[macro_use] extern crate lazy_static;
extern crate regex;
use std::{fs};
use regex::Regex;


/// A passport has 8 fields, of which not all might be present
#[derive(Debug)]
struct Passport {
    byr : Option<String>, // Birth Year
    iyr : Option<String>, // Issue Year
    eyr : Option<String>, // Expiration Year
    hgt : Option<String>, // Height
    hcl : Option<String>, // Hair Color
    ecl : Option<String>, // ye Color
    pid : Option<String>, // Passport ID
    cid : Option<String>  // Country ID
}

impl Passport {
    /// Accept passports as valid according to task 1.
    ///
    /// All fields except cid MUST be present.
    fn is_valid_1(&self) -> bool {
        match self {
            Passport{byr: Some(_), iyr: Some(_), eyr: Some(_), hgt: Some(_), hcl: Some(_), ecl: Some(_), pid: Some(_), ..} => true,
            _ => false
        }
    }

    /// Accept passports as valid according to task 2.
    ///
    /// All field except cid MUST be present as in task 1,
    /// additionally, fields get individually validated.
    fn is_valid_2(&self) -> bool {

        /// nbr in between min (included) and max (included).
        fn inbetween(nbr: i32, min :i32, max:i32) -> bool {
            (min <= nbr) & (nbr <= max)
        }

        /// Validate individual fields.
        ///
        /// fieldname: name of field in passwort
        /// field: refernce to String in Password
        fn validate_field( fieldname : &str, field : &String) -> bool {
            match fieldname {
                "byr" => { // byr (Birth Year) - four digits; at least 1920 and at most 2002.
                    if let Ok(year) = field.parse::<i32>() { inbetween(year, 1920, 2002) }
                    else { false }
                },
                "iyr" => { // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
                    if let Ok(year) = field.parse::<i32>() { inbetween(year, 2010, 2020) }
                    else { false }
                },
                "eyr" => { // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
                    if let Ok(year) = field.parse::<i32>() { inbetween(year, 2020, 2030) }
                    else { false }
                },
                "hgt" => {
                    // hgt (Height) - a number followed by either cm or in:
                    // If cm, the number must be at least 150 and at most 193.
                    // If in, the number must be at least 59 and at most 76.
                    // @todo pattern match on end of string
                    if field.ends_with("cm") {
                        if let Ok(hgt) = field[..field.len()-2].parse::<i32>() { inbetween(hgt, 150, 193) }
                        else { false }

                    }
                    else if field.ends_with("in") {
                        if let Ok(hgt) = field[..field.len()-2].parse::<i32>() { inbetween(hgt, 59, 76) }
                        else { false }
                    }
                    else { false }
                },
                "hcl" => {
                    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
                    // Avoid recompiling the regex by every call of the function
                    // Compile it only on the first use
                    lazy_static! {
                        static ref RE: Regex = Regex::new("^#[[a-f][0-9]]{6}$").unwrap();
                    }
                    RE.is_match(field)
                }
                "ecl" => { // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
                    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|e| e == field)
                }
                "pid" => {
                    // pid (Passport ID) - a nine-digit number, including leading zeroes.
                    // Avoid recompiling the regex by every call of the function
                    // Compile it only on the first use
                    lazy_static! {
                        static ref RE: Regex = Regex::new("^[0-9]{9}$").unwrap();
                    }
                    RE.is_match(field)
                }
                _ => false
            }
        }


        match self {
            Passport{byr: Some(byr), iyr: Some(iyr), eyr: Some(eyr),
                hgt: Some(hgt), hcl: Some(hcl), ecl: Some(ecl),
                pid: Some(pid), ..} => {
                validate_field("byr", byr) &
                    validate_field("iyr", iyr) &
                    validate_field("eyr", eyr) &
                    validate_field("hgt", hgt) &
                    validate_field("hcl", hcl) &
                    validate_field("ecl", ecl) &
                    validate_field("pid", pid)
            },
            _ => false
        }
    }
}


impl From<String> for Passport {
    /// Generate passport specifically from string as provided by the aoc.
    fn from(s: String) -> Passport {
        let mut p = Passport{
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None
        };
        let v: Vec<&str> = s.split(|x| (x == ' ') || (x == '\n')).collect();
        for entry in v {
            match entry.get(..3) {
                Some("byr") => { p.byr = Some(entry[4..].to_string()); },
                Some("iyr") => { p.iyr = Some(entry[4..].to_string()); },
                Some("eyr") => { p.eyr = Some(entry[4..].to_string()); },
                Some("hgt") => { p.hgt = Some(entry[4..].to_string()); },
                Some("hcl") => { p.hcl = Some(entry[4..].to_string()); },
                Some("ecl") => { p.ecl = Some(entry[4..].to_string()); },
                Some("pid") => { p.pid = Some(entry[4..].to_string()); },
                Some("cid") => { p.cid = Some(entry[4..].to_string()); },
                 _ => () // Do nothing
            };
        }
        p
    }
}


fn main() {
    let data = fs::read_to_string("input/input04.txt").expect("Unable to read file");
    let mut counter_1: i32 = 0; // passports that fulfill criteria from task 1
    let mut counter_2: i32 = 0; // passports that fulfill criteria from task 2
    for pass_candidate in data.split("\n\n"){
        let p = Passport::from(pass_candidate.to_string());
        if p.is_valid_1() {
            counter_1 +=1;
        }
        if p.is_valid_2() {
            counter_2 +=1;
        }
    }
    println!("First task {}", counter_1);
    println!("Second task {}", counter_2);
}
