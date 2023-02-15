use std::fs;    

pub fn main() {
    let input = fs::read_to_string("../../1in.txt").expect("Should have been able to read the file");
   
    let elfs_calories_str: Vec<&str> = input.split("\r\n\r\n").collect();
    let elfs_calories: Vec<Vec<i32>> = elfs_calories_str.iter().map(|&val| {
        let elf_calories_str: Vec<&str> = val.split("\r\n").collect();
        let elf_calories: Vec<i32> = elf_calories_str.iter().map(|&val| {
            return val.parse::<i32>().unwrap();
        }).collect();
        return elf_calories;
    }).collect();
    part1(&elfs_calories);
    part2(&elfs_calories);

    fn part1(elfs_calories: &Vec<Vec<i32>>) {
        let mut max = 0;

        for list in elfs_calories {
            let sum: i32 = list.iter().sum();            
            if sum > max {
                max = sum;
            }
        }
        println!("{}", max)
    }

    fn part2(elfs_calories: &Vec<Vec<i32>>){
        let mut list_of_sums: Vec<i32> = vec![0];

        for list in elfs_calories {
            let sum: i32 = list.iter().sum();  
            list_of_sums.push(sum);
        }

        list_of_sums.sort();
        list_of_sums.reverse();
        let final_sum : i32 = list_of_sums[0] + list_of_sums[1] + list_of_sums[2];
        println!("{}", final_sum);
    }

}




