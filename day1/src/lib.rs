use std::{
    fs::File,
    io::{Read, Result},
};

pub fn part_1(file_contents: &String) -> isize {
    let mut increase_count = 0;
    let mut prev_num = f64::INFINITY as isize;

    file_contents.lines().for_each(|line| {
        let current_num = parse_number(line);

        if current_num > prev_num {
            increase_count += 1;
        }

        prev_num = current_num;
    });

    increase_count
}

pub fn part_2(file_contents: &String) -> isize {
    let mut prev_sum = f64::INFINITY as isize;
    let mut increase_count = 0;

    let lines = file_contents
        .lines()
        .map(parse_number)
        .collect::<Vec<isize>>();

    lines.windows(3).for_each(|numbers| {
        let current_sum: isize = numbers.iter().sum();
        if current_sum > prev_sum {
            increase_count += 1;
        }
        prev_sum = current_sum;
    });

    increase_count
}

pub fn read_file(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn parse_number(line: &str) -> isize {
    match line.parse::<isize>() {
        Ok(n) => n,
        Err(_) => 0,
    }
}

#[cfg(test)]
mod day1_testsc {
    #[test]
    fn test_part_1() {
        let file_contents = crate::read_file("testinput.txt").unwrap();
        assert_eq!(crate::part_1(&file_contents), 7);
    }

    #[test]
    fn test_part_2() {
        let file_contents = crate::read_file("testinput.txt").unwrap();
        assert_eq!(crate::part_2(&file_contents), 5);
    }
}
