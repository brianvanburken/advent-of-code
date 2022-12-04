use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read file");

    let pairs: Vec<Vec<isize>> = input
        .lines()
        .map(|pair| {
            pair.split(",")
                .flat_map(|assignment| {
                    assignment
                        .split("-")
                        .map(|section| section.parse().expect("Expected assignment to be a number"))
                        .collect::<Vec<isize>>()
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut sum_part_1 = 0;
    let mut sum_part_2 = 0;
    for pair in pairs {
        let [s1,e1,s2,e2] = &pair[..] else {
                panic!("Expected a pair with assignment resulting in four numbers")
            };
        if (s1 <= s2 && e1 >= e2) || (s1 >= s2 && e1 <= e2) {
            sum_part_1 += 1;
        }
        if e1 >= s2 && e2 >= s1 {
            sum_part_2 += 1;
        }
    }
    println!("Part 1: {:?}", sum_part_1);
    println!("Part 2: {:?}", sum_part_2);
}
