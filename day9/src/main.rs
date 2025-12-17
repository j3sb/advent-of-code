use rayon::prelude::*;
use std::fs;

fn area((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> i64 {
    let aa = (x1.min(x2), y1.min(y2));
    let bb = (x1.max(x2), y1.max(y2));

    let area = (bb.0 - aa.0 + 1) * (bb.1 - aa.1 + 1);
    return area;
}

fn is_between(a: (i64, i64), b: (i64, i64), pos: (i64, i64)) -> bool {
    let top;
    let bot;
    let mid;
    if a.0 == b.0 {
        if pos.0 != a.0 {
            return false;
        }
        top = a.1.max(b.1);
        bot = a.1.min(b.1);
        mid = pos.1;
    } else {
        if pos.1 != a.1 {
            return false;
        }
        top = a.0.max(b.0);
        bot = a.0.min(b.0);
        mid = pos.0;
    }

    return bot <= mid && mid <= top;
}

fn is_between_vert(a: (i64, i64), b: (i64, i64), pos: (i64, i64)) -> bool {
    let top;
    let bot;
    let mid;

    if a.0 < pos.0 || b.0 < pos.0 {
        if a.0 == b.0 {
            // vertical
            top = a.1.max(b.1);
            bot = a.1.min(b.1);
            mid = pos.1;

            return bot <= mid && mid < top;
        } else {
            // horizontal

            if a.1 == pos.1 && (a.0 > pos.0 || b.0 > pos.0) {
                return true;
            } else {
                return false;
            }
        }
    } else {
        return false;
    }
}

fn is_inside(pos: (i64, i64), positions: &Vec<(i64, i64)>) -> bool {
    let mut intersections: u64 = 0;

    let first_pos = positions[0];
    let last_pos = positions[positions.len() - 1];

    if is_between_vert(first_pos, last_pos, pos) {
        intersections += 1;
    }

    if is_between(first_pos, last_pos, pos) {
        return true;
    }

    for i in 0..(positions.len() - 1) {
        let a = positions[i];
        let b = positions[i + 1];

        if is_between_vert(a, b, pos) {
            intersections += 1;
        }

        if is_between(a, b, pos) {
            return true;
        }
    }

    // return false;
    return intersections % 2 == 1;
}

static mut COUNTER: u64 = 0;

fn is_valid((x1, y1): (i64, i64), (x2, y2): (i64, i64), positions: &Vec<(i64, i64)>) -> bool {
    let left = x1.min(x2);
    let right = x1.max(x2);
    let bot = y1.min(y2);
    let top = y1.max(y2);

    let top_left = (left, top);
    let top_right = (right, top);
    let bot_left = (left, bot);
    let bot_right = (right, bot);

    unsafe {
        println!("validiting {}", *&raw const COUNTER);
        COUNTER += 1;
    }

    // corners are inside
    if !(is_inside(top_left, positions)
        && is_inside(top_right, positions)
        && is_inside(bot_left, positions)
        && is_inside(bot_right, positions))
    {
        return false;
    }

    // serial implementation
    // // top and bottom line
    // for x in left..right {
    //     let top_line = (x, top);
    //     let bot_line = (x, bot);

    //     if !(is_inside(top_line, positions) && is_inside(bot_line, positions)) {
    //         return false;
    //     }
    // }

    // // left and right line
    // for y in bot..top {
    //     let left_line = (left, y);
    //     let right_line = (right, y);

    //     if !(is_inside(left_line, positions) && is_inside(right_line, positions)) {
    //         return false;
    //     }
    // }

    // return true;

    // parallel implementation
    let horizontal = (left..right).into_par_iter().map(|x| ((x, top), (x, bot)));
    let vertical = (bot..top).into_par_iter().map(|y| ((left, y), (right, y)));

    return horizontal
        .chain(vertical)
        .into_par_iter()
        .all(|(a, b)| is_inside(a, positions) && is_inside(b, positions));
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("input file");

    let positions: Vec<(i64, i64)> = data
        .split("\n")
        .map(|s| {
            let mut p = s.split(",").map(|v| v.parse().expect("a usize"));

            let a = p.next().expect("first number");
            let b = p.next().expect("first number");

            (a, b)
        })
        .collect();

    let max = positions
        .iter()
        .flat_map(|(a, b)| [a, b])
        .max()
        .expect("a max value")
        + 2;

    let areas: Vec<Vec<i64>> = positions
        .iter()
        .map(|p1| positions.iter().map(|p2| area(*p1, *p2)).collect())
        .collect();

    println!(
        "areas: {}",
        areas.iter().flatten().collect::<Vec<&i64>>().len(),
    );
    println!("max coord: {max}");

    let allowed_areas: Vec<Vec<i64>> = areas
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, _a)| is_valid(positions[*x], positions[y], &positions))
                .map(|(_x, a)| *a)
                .collect()
        })
        .collect();

    // for y in 0..max {
    //     for x in 0..max {
    //         if is_inside((x as i64, y as i64), &positions) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     print!("\n");
    // }

    let max_area = allowed_areas
        .iter()
        .flatten()
        .max()
        .expect("some max value");

    println!("max area: {max_area}");

    // println!("{:?}", areas);
    // println!("{:?}", allowed_areas);
}
