use std::{fs, vec};

fn main() {
    let data = fs::read_to_string("./input.txt").expect("read the file");

    let batteries = data.split("\n");

    let mut sum = 0;

    for battery in batteries {
        let mut maxes: Vec<u32> = vec![];
        let mut rest: Box<String> = Box::from(battery.to_string());

        for i in (0..12).rev() {
            let without_lasts: String = rest.chars().take(rest.chars().count() - i).collect();

            let (max_index, max) = without_lasts
                .chars()
                .collect::<Vec<_>>()
                .iter()
                .enumerate()
                .rev()
                .map(|(i, c)| (i, c.to_digit(10).expect("a digit")))
                .max_by_key(|&(_, d)| d)
                .expect("non empty line");

            let battery2: Box<String> =
                Box::from(rest.chars().skip(max_index + 1).collect::<String>());

            rest = battery2;

            maxes.push(max);
        }

        let max_joltage = maxes.iter().rev().enumerate().fold(0, |acc, (i, m)| {
            acc + (*m as usize) * ((10 as usize).pow(i as u32))
        });

        println!("{max_joltage}");
        sum += max_joltage;
    }

    println!("{sum}");
}
