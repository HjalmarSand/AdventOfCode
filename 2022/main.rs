use std::fs;
use std::collections::HashSet;
use std::hash::Hash;



fn main() {

    part1();
    part2();
}

fn part2() {
    
    
    let input = fs::read_to_string("../../9in.txt").expect("Should have been able to read the file");
    let moves: Vec<&str>  = input.trim().split("\n").collect();

    let mut x_head: i32 = 0;
    let mut y_head: i32 = 0;
    
    let mut visited: HashSet<(i32,i32)> = HashSet::new();
    visited.insert((0,0));

    let mut points: Vec<(i32, i32, &str)> = vec![];
    let mut counter : i32 = 10;
    while counter > 0 {
        points.push((0,0,"O"));
        counter = counter - 1;
    }

    for el in moves {
        let (temp1, temp2) = el.trim().split_at(el.trim().len()/2);
        let (dir, mut len) = (temp1.trim(), temp2.trim().parse::<i32>().expect("HEJHEJ"));

        while len > 0 {
            if dir == "U" {
                y_head = y_head + 1;
            
            } else if dir == "R" {
                x_head = x_head + 1;
               
            } else if dir == "D" {
                y_head = y_head - 1;
               
            } else {
                x_head = x_head - 1;
                
            }
            len = len - 1;
        }
    }
}

fn part1() {
    
    
    let input = fs::read_to_string("../../9in.txt").expect("Should have been able to read the file");
    let moves: Vec<&str>  = input.trim().split("\n").collect();

    let mut x_head: i32 = 0;
    let mut y_head: i32 = 0;
    let mut x_tail: i32 = 0;
    let mut y_tail: i32 = 0;
    let mut visited: HashSet<(i32,i32)> = HashSet::new();
    visited.insert((0,0));

    for el in moves {
        let (temp1, temp2) = el.trim().split_at(el.trim().len()/2);
        let (dir, mut len) = (temp1.trim(), temp2.trim().parse::<i32>().expect("HEJHEJ"));

        while len > 0 {
            if dir == "U" {
                y_head = y_head + 1;
                if is_neighbour(x_head, y_head, x_tail, y_tail) {
                    
                } else {

                    y_tail = y_head - 1;
                    x_tail = x_head;
                    visited.insert((x_tail, y_tail));
                }

            } else if dir == "R" {
                x_head = x_head + 1;
                if is_neighbour(x_head, y_head, x_tail, y_tail) {
                    
                } else {
                    x_tail = x_head - 1;
                    y_tail = y_head;
                    visited.insert((x_tail, y_tail));
                }

            } else if dir == "D" {
                y_head = y_head - 1;
                if is_neighbour(x_head, y_head, x_tail, y_tail) {
                    
                } else {
                    y_tail = y_head + 1;
                    x_tail = x_head;
                    visited.insert((x_tail, y_tail));
                }
            } else {
                x_head = x_head - 1;
                if is_neighbour(x_head, y_head, x_tail, y_tail) {
                    
                } else {
                    x_tail = x_head + 1;
                    y_tail = y_head;
                    visited.insert((x_tail, y_tail));
                }
            }
            len = len - 1;
        }
    }

    println!("{}" , visited.len());
}

fn is_neighbour(hx: i32, hy: i32, tx: i32, ty: i32)  -> bool {
    if hx.abs_diff(tx) > 1 || hy.abs_diff(ty) > 1 {
        return false;
    } else {
        return true;
    }
}

fn move_all<'a>(new_dir : &str, points: &'a mut Vec<(i32,i32,&str)>) {
    let mut size = points.len();
    while size > 0 {
        if points.len() - size == 0 {
            points[0].2 = new_dir;
        }
        let mut point = points[points.len() - size];

        size = size - 1;
    } 
}