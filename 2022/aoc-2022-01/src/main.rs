use std::fs;

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");

    let mut highest_total = 0;
    let mut totals: Vec<usize> = vec![];
    let mut current_elf_calories = 0;

    for line in data.lines().into_iter() {
        if line.trim() == "" {
            totals.push(current_elf_calories);
            highest_total = if current_elf_calories > highest_total {
                current_elf_calories
            } else {
                highest_total
            };
            current_elf_calories = 0;
        } else {
            let calories = line.parse::<usize>().unwrap();
            current_elf_calories += calories;
        }
    }

    println!("Part 1: {:?}", highest_total);

    totals.sort();
    totals.reverse();

    let top_3: usize = totals[0..3].iter().sum();

    println!("Part 2: {:?}", top_3);
}
