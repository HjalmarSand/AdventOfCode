use std::fs;

fn main() {
    let input = fs::read_to_string("../../2in.txt").expect("Should have been able to read the file");
    let moves: Vec<&str>  = input.split("\n").collect();
    let mut their_moves: Vec<&str> = vec![];
    let mut our_moves: Vec<&str> = vec![];
    let mut score: i32 = 0;

    for el in moves {
        
        let temp : Vec<&str> = el.trim().splitn(2, ' ').collect();
        their_moves.push(temp[0]);
        our_moves.push(temp[1]);
    }
    
    for el in their_moves {
        let temp = our_moves[0];
        our_moves.remove(0);

        if el == "A" {
            if temp == "Y" {
                score = score + 4;
            } else if temp == "X" {
                score = score + 3;
            } else {
                score = score + 8;
            }
        } else if el == "B" {
            if temp == "Y" {
                score = score + 5;
            } else if temp == "X" {
                score = score + 1;
            } else {
                score = score + 9;
            }
        } else {
            if temp == "Y" {
                score = score + 6;
            } else if temp == "X" {
                score = score + 2;
            } else {
                score = score + 7;
            }
        }
              
    }

    println!("{score}");
    
}
