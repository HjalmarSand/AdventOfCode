use std::{fs, collections::HashMap};

fn main() {

    part1();
    part2();

    
}

fn ascii_converter(c:char) -> u32 { 
    let map:HashMap<char,u32> = ('a'..='z').chain('A'..='Z').zip(1..).collect();

    return *map.get(&c).unwrap();

}

fn part2() {
    let input = fs::read_to_string("../../3in.txt").expect("Should have been able to read the file");
    let mut bags: Vec<&str>  = input.trim().split("\n").collect();
    let mut incorrect_item: Vec<char> = vec![];
    
    while bags.len() != 0 {  

        let first_bag:Vec<char> = bags.pop().unwrap().chars().collect();
        
        let second_bag: Vec<char> = bags.pop().unwrap().chars().collect();
        
        let third_bag: Vec<char> =  bags.pop().unwrap().chars().collect();
        

        for item in first_bag {
            if second_bag.contains(&item) && third_bag.contains(&item){
                incorrect_item.push(item);
                break;
            }
        }
    }

    let mut sum: u32 = 0;
    for el in incorrect_item {
        sum = sum + ascii_converter(el);
    }

    println!("{sum}");
}

fn part1() {

    let input = fs::read_to_string("../../3in.txt").expect("Should have been able to read the file");
    let bags: Vec<&str>  = input.trim().split("\n").collect();
    let mut incorrect_item: Vec<char> = vec![];
    
    for el in bags {    
        let (first_half, second_half) = el.trim().split_at(el.trim().len()/2);
        
        let leftchar_vec: Vec<char> = first_half.chars().collect(); 
        let rightchar_vec: Vec<char> = second_half.chars().collect(); 

        for item in leftchar_vec {
            if rightchar_vec.contains(&item) {
                incorrect_item.push(item);
                break;
            }
        }
    }

    let mut sum: u32 = 0;
    for el in incorrect_item {
        sum = sum + ascii_converter(el);
    }

    println!("{sum}");
}
