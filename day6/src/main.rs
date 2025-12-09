use core::num;
use std::fs;

fn main() {
    let data = fs::read_to_string("./input.txt").expect("input file");

    let mut line_data: Vec<&str> = data.split("\n").collect();
    let height = line_data.len();

    let operations = line_data.pop().expect("a last row with operations");

    let mut total_sum = 0;
    let mut last_op: char = ' ';
    let mut prod = 1usize;
    let mut sum = 0usize;

    for (x, op) in operations.chars().enumerate() {
        if op != ' ' {
            if last_op == '*' {
                println!("prod is {prod}");
                total_sum += prod;
                prod = 1;
            } else {
                println!("sum is {sum}");
                total_sum += sum;
                sum = 0;
            }

            last_op = op;
        }

        let number_string: String = line_data
            .iter()
            .map(|s| s.chars().nth(x).expect("a char"))
            .filter(|c| *c != ' ')
            .collect::<String>();
        // println!("string: {number_string}, op: {op}");

        if number_string == "" {
            continue;
        }
        let number: usize = number_string.parse().expect("a usize in string form");
        println!("usize: {number}, lastop: {last_op}");

        if last_op == '+' {
            sum += number;
        } else {
            prod *= number;
        }
    }

    if last_op == '*' {
        println!("prod is {prod}");
        total_sum += prod;
        prod = 1;
    } else {
        println!("sum is {sum}");
        total_sum += sum;
        sum = 0;
    }

    println!("{total_sum}");
}

// first part
// fn main() {
//     let data = fs::read_to_string("./input.txt").expect("input file");

//     let line_data: Vec<&str> = data.split("\n").collect();
//     let height = line_data.len();

//     let mut table: Vec<Vec<&str>> = line_data
//         .iter()
//         .map(|s| s.split(" ").filter(|e| *e != "").collect())
//         .collect();

//     let operations = table.pop().expect("last row with operations");

//     let usize_table: Vec<Vec<usize>> = table
//         .iter()
//         .map(|row| row.iter().map(|s| s.parse().expect("a usize")).collect())
//         .collect();

//     let width = table[0].len();

//     let mut total_sum = 0usize;
//     for (x, op) in operations.iter().enumerate() {
//         if *op == "*" {
//             let mut prod = 1usize;
//             for y in 0..usize_table.len() {
//                 prod *= usize_table[y][x];
//             }
//             total_sum += prod;
//         }
//         if *op == "+" {
//             let mut sum = 0usize;
//             for y in 0..usize_table.len() {
//                 sum += usize_table[y][x];
//             }
//             total_sum += sum;
//         }
//     }

//     println!("{:?}", usize_table);
//     println!("{width}, {height}");
//     println!("{total_sum}");
// }
