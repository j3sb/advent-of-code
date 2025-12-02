use core::slice::Split;
use std::{fs, thread::JoinHandle};

fn isInvalid(s: &str, p_length: usize) -> bool {
    let c: Vec<char> = s.chars().collect();

    if s.len() % p_length != 0 {
        return false;
    }

    let repeats = s.len() / p_length;
    for i in 0..p_length {
        for j in 1..repeats {
            if c[i] != c[i + j * p_length] {
                return false;
            }
        }
    }

    return true;
}

fn main() {
    let data = fs::read_to_string("./input.txt").expect("read the file");

    let intervals: Vec<Vec<&str>> = data.split(",").map(|s| s.split("-").collect()).collect();
    let mut sum = 0;

    for interval in intervals {
        println!("{}, {}", interval[0], interval[1]);
        let min: usize = interval[0].parse().expect("a number");
        let max: usize = interval[1].parse().expect("a number");

        for i in min..=max {
            let s = i.to_string();
            for p_length in 1..s.len() {
                if isInvalid(&s, p_length) {
                    if p_length != s.len() / 2 {
                        println!("{i}, is invalid, of length {p_length}");
                    }
                    sum += i;
                    break;
                }
            }
        }
    }

    println!("{sum}");
}

// part one
// fn isInvalid(i: u64) -> bool {
//     let s = i.to_string();
//     let c: Vec<char> = s.chars().collect();

//     if s.len() % 2 == 1 {
//         return false;
//     }

//     let mid = s.len() / 2;

//     for i in 0..mid {
//         if c[i] != c[i + mid] {
//             return false;
//         }
//     }

//     return true;
// }

// fn main() {
//     let data = fs::read_to_string("./input.txt").expect("read the file");

//     let intervals: Vec<Vec<&str>> = data.split(",").map(|s| s.split("-").collect()).collect();
//     let mut sum = 0;

//     for interval in intervals {
//         println!("{}, {}", interval[0], interval[1]);
//         let min: u64 = interval[0].parse().expect("a number");
//         let max: u64 = interval[1].parse().expect("a number");

//         for i in min..=max {
//             if isInvalid(i) {
//                 println!("{i}, is invalid");
//                 sum += i;
//             }
//         }
//     }

//     println!("{sum}");
// }
