use std::{fs};

fn populate_field(field : &mut Vec<Vec<i32>>, data_arr: &Vec<&str>){
    for y in 0..field.len() {
        for x in 0..field[0].len() {
            if data_arr[y].chars().nth(x).unwrap() == '#' {
                field[y][x] = 1
            }
        }
    }
}

fn step(pos_x : i32, pos_y : i32, step_x : i32, step_y : i32, dim_x: i32) ->  (i32, i32) {
    let mut x_new = pos_x + step_x;
    if x_new >= dim_x { x_new -= dim_x; }
    (x_new , pos_y+step_y)
}


fn runslopes(field : &mut Vec<Vec<i32>>, step_x: i32, step_y: i32, x_dim : usize, y_dim: usize) -> i64{
    let mut pos_x : i32 = 0;
    let mut pos_y : i32 = 0;
    let mut counter = 0;
    while pos_y < (y_dim as i32 -1) {
        let (npos_x, npos_y) = step(pos_x, pos_y, step_x, step_y, x_dim as i32);
        pos_y = npos_y;
        pos_x = npos_x;
        if field[pos_y as usize][pos_x as usize] == 1 {
            counter += 1;
        }

    }
    counter
}

fn main() {
    let data = fs::read_to_string("input/input03.txt").expect("Unable to read file");
    let data_arr : Vec<&str> = data.lines().collect();
    let y_dim = data_arr.len();
    let x_dim = data_arr[0].len();
    let mut field = vec![vec![0; x_dim]; y_dim];
    populate_field(&mut field,&data_arr);

    println!("First task: {}", runslopes(&mut field, 3,1,x_dim,y_dim));

    let mut task2 :i64 = 1;
    task2 *= runslopes(&mut field,1,1,x_dim,y_dim);
    task2 *= runslopes(&mut field,3,1,x_dim,y_dim);
    task2 *= runslopes(&mut field,5,1,x_dim,y_dim);
    task2 *= runslopes(&mut field,7,1,x_dim,y_dim);
    task2 *= runslopes(&mut field,1,2,x_dim,y_dim);
    println!("Second task: {}", task2)
}
