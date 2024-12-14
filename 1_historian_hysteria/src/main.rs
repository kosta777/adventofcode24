use std::env;
use std::fs;
use std::io::BufReader;
use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    
    //? unpacks the return value, if its Ok(val) returns val, otherwise returns error right away to
    //the caller
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    
    let mut numbers_left: Vec<u32> = Vec::new();
    let mut numbers_right: Vec<u32> = Vec::new();
    for line in reader.lines() {
        let line = line?; // Handle possible IO errors per line
        let numbers: Vec<u32> = line
            .split_whitespace() 
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        if numbers.len() == 2 {
            numbers_left.push(numbers[0]);
            numbers_right.push(numbers[1]);
            //println!("Number 1: {}, Number 2: {}", numbers[0], numbers[1]);
        } else {
            eprintln!("Invalid line format: {}", line);
        }

    }

    numbers_left.sort();
    numbers_right.sort();
    
    let mut sum=0;
    for i in 0..numbers_left.len() {
        sum += numbers_left[i].abs_diff(numbers_right[i]);
    }

    println!("{}", sum);


    Ok(())
}
