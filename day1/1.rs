use std::fs;

fn main() {
    let rotations_string = fs::read_to_string("./1.txt").expect("read the file");

    let mut p: i64 = 50;
    let mut count: i64 = 0;

    let mut rotations = rotations_string.split("\n").collect::<Vec<_>>();
    rotations.pop();

    for rotation in rotations {
        let val = rotation[1..].parse::<i64>().expect("a number");
        let dir = if rotation.chars().nth(0).expect("at least 2 chars") == 'L' {
            -1
        } else {
            1
        };

        for _i in 0..val {
            p += 100 + dir;
            p = p % 100;

            if p < 0 {
                println!("{p}");
            }

            if p == 0 {
                count += 1;
            }
        }
    }

    println!("{count}");
}
