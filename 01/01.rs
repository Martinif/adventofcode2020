use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line
            .and_then(|v| v
                .parse()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))
                )
            )
        .collect()
}

fn sum2to2020(input: &Vec<i64>) -> i64 {
    for i in input {
        if input.contains(&(2020-i)) { return i*(2020-i) }
    }
    0
}

fn sum3to2020(input: &Vec<i64>) -> i64 {
    for i in input {
        for j in input {
            if input.contains(&(2020-i-j)) {return i * j * (2020-i-j)}
        }
    }
    0
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input1.txt")?)?;
    println!("First task: {:#?}", sum2to2020(&vec));
    println!("Second task: {:#?}", sum3to2020(&vec));

    Ok(())
}