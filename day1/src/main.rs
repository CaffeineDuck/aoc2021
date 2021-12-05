use day1::{read_file, part_1, part_2};

fn main() {
    let file_contents = read_file("input.txt").unwrap();

    println!("Increase count for PART 1: {}", part_1(&file_contents));
    println!("Increase count for PART 2: {}", part_2(&file_contents));
}

