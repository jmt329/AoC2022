use std::{fs, str::FromStr};

fn split_inventory(file_lines: std::str::Lines) -> Result<Vec<Vec<u32>>, <u32 as FromStr>::Err> {
    let mut inventory = Vec::new();
    let mut elf_inventory = Vec::new();

    for line in file_lines {
        if !line.is_empty() {
            elf_inventory.push(line.parse::<u32>()?);
        } else {
            inventory.push(elf_inventory.clone());
            elf_inventory = Vec::new();
        }
    }
    inventory.push(elf_inventory.clone());
    Ok(inventory)
}

fn main() {
    // read in input
    let file_string = fs::read_to_string("input").unwrap();
    let file_lines = file_string.lines();

    // get a vector of calories for each elf
    let elves = split_inventory(file_lines).unwrap();
    let mut sums: Vec<u32> = elves.iter().map(|x| x.iter().sum()).collect();

    // answer puzzles
    sums.sort();
    let max = sums.last().unwrap();
    println!("max = {}", max);
    let top3: u32 = sums.get(sums.len() - 3..sums.len()).unwrap().iter().sum();
    println!("top3 = {}", top3);
}
