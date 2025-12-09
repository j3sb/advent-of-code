use std::{fs, mem};

trait SafeSetting {
    fn safe_add(&mut self, i: usize, v: usize) -> ();
}

impl SafeSetting for Vec<usize> {
    fn safe_add(&mut self, i: usize, v: usize) -> () {
        if i >= self.len() {
            return;
        }

        self[i] += v;
    }
}

fn main() {
    let data = fs::read_to_string("./input.txt").expect("an input file");

    let rows: Vec<&str> = data.split("\n").collect();
    let splitters: Vec<Vec<bool>> = rows
        .iter()
        .map(|s| s.chars().map(|c| c == '^').collect())
        .collect();

    let start = rows[0].find('S').expect("an S in the first row");

    println!("{:?}", splitters);
    println!("{start}");

    let mut current_row = vec![0usize; rows[0].len()];
    current_row[start] = 1;

    let mut splits = 0usize;

    for row in splitters {
        let mut new_row = current_row.clone();
        println!("{:?}", current_row);
        for (i, s) in row.iter().enumerate() {
            if *s && current_row[i] != 0 {
                new_row.safe_add(i - 1, current_row[i]);
                new_row.safe_add(i + 1, current_row[i]);
                new_row[i] = 0;
                splits += 1;
            }
        }

        mem::swap(&mut current_row, &mut new_row);
    }
    println!("{:?}", current_row);

    println!("{splits}");

    let sum: usize = current_row.iter().sum();
    println!("{sum}");
}

// failed attempt at part 2 (again)
// fn dfs(current_row: usize, row_index: usize, rows: &Vec<Vec<bool>>) -> usize {
//     if row_index >= rows.len() {
//         return 1;
//     }

//     let row = &rows[row_index];
//     let mut sum = 0;

//     for (i, s) in row.iter().enumerate() {
//         if current_row == i {
//             if *s {
//                 sum += dfs(i - 1, row_index + 1, rows);
//                 sum += dfs(i + 1, row_index + 1, rows);
//             } else {
//                 sum += dfs(i, row_index + 1, rows);
//             }
//         }
//     }

//     return sum;
// }

// fn main() {
//     let data = fs::read_to_string("./input.txt").expect("an input file");

//     let rows: Vec<&str> = data.split("\n").collect();
//     let splitters: Vec<Vec<bool>> = rows
//         .iter()
//         .map(|s| s.chars().map(|c| c == '^').collect())
//         .collect();

//     let start = rows[0].find('S').expect("an S in the first row");

//     let v = dfs(start, 0, &splitters);
//     println!("{v}");
// }

// failed attempt at part 2
// #[allow(dead_code)]

// fn print_row(row: &Vec<bool>) {
//     let s: String = row.iter().map(|b| if *b { '|' } else { '.' }).collect();
//     println!("{s}");
// }

// fn main() {
//     let data = fs::read_to_string("./input.txt").expect("an input file");

//     let rows: Vec<&str> = data.split("\n").collect();
//     let splitters: Vec<Vec<bool>> = rows
//         .iter()
//         .map(|s| s.chars().map(|c| c == '^').collect())
//         .collect();

//     let start = rows[0].find('S').expect("an S in the first row");

//     let width = rows[0].len();

//     let mut current_rows = vec![start];

//     for (x, row) in splitters.iter().enumerate() {
//         let mut new_rows: Vec<usize> = vec![];

//         for current_row in &current_rows {
//             for (i, s) in row.iter().enumerate() {
//                 if *current_row == i {
//                     if *s {
//                         if i - 1 < width {
//                             new_rows.push(i - 1);
//                         }
//                         if i + 1 < width {
//                             new_rows.push(i + 1);
//                         }
//                     } else {
//                         new_rows.push(*current_row);
//                     }
//                 }
//             }
//         }
//         mem::swap(&mut current_rows, &mut new_rows);
//         println!("row: {x}, {}", current_rows.len());
//     }

//     println!("{}", current_rows.len());
// }

// part 1
// trait SafeSetting {
//     fn safe_set(&mut self, i: usize, v: bool) -> ();
// }

// impl SafeSetting for Vec<bool> {
//     fn safe_set(&mut self, i: usize, v: bool) -> () {
//         if i >= self.len() {
//             return;
//         }

//         self[i] = v;
//     }
// }

// fn print_row(row: &Vec<bool>) {
//     let s: String = row.iter().map(|b| if *b { '|' } else { '.' }).collect();
//     println!("{s}");
// }

// fn main() {
//     let data = fs::read_to_string("./input.txt").expect("an input file");

//     let rows: Vec<&str> = data.split("\n").collect();
//     let splitters: Vec<Vec<bool>> = rows
//         .iter()
//         .map(|s| s.chars().map(|c| c == '^').collect())
//         .collect();

//     let start = rows[0].find('S').expect("an S in the first row");

//     println!("{:?}", splitters);
//     println!("{start}");

//     let mut current_row = vec![false; rows[0].len()];
//     current_row[start] = true;

//     let mut splits = 0usize;

//     for row in splitters {
//         let mut new_row = current_row.clone();
//         print_row(&current_row);
//         for (i, s) in row.iter().enumerate() {
//             if *s && current_row[i] {
//                 new_row.safe_set(i - 1, true);
//                 new_row.safe_set(i + 1, true);
//                 new_row[i] = false;
//                 splits += 1;
//             }
//         }

//         mem::swap(&mut current_row, &mut new_row);
//     }
//     print_row(&current_row);

//     println!("{splits}");
// }
