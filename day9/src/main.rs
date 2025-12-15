// use stacksafe::stacksafe;
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
    if a.0 == b.0 {
        if pos.0 != a.0 {
            return false;
        }
        top = a.1.max(b.1);
        bot = a.1.min(b.1);
        mid = pos.1;
    } else {
        return false;
    }

    return bot <= mid && mid <= top;
}

fn _is_green_old(positions: &Vec<(i64, i64)>, pos: (i64, i64)) -> bool {
    let first_pos = positions[0];
    let last_pos = positions[positions.len() - 1];

    if is_between(first_pos, last_pos, pos) {
        return true;
    }

    for i in 0..(positions.len() - 1) {
        let a = positions[i];
        let b = positions[i + 1];

        if is_between(a, b, pos) {
            return true;
        }
    }

    return false;
}

fn is_green(positions: &Vec<(i64, i64)>, pos: (i64, i64)) -> bool {
    let first_pos = positions[0];
    let last_pos = positions[positions.len() - 1];

    if is_between_vert(first_pos, last_pos, pos) {
        return true;
    }

    for i in 0..(positions.len() - 1) {
        let a = positions[i];
        let b = positions[i + 1];

        if is_between_vert(a, b, pos) {
            return true;
        }
    }

    return false;
}

fn _is_inside_old(pos: (i64, i64), positions: &Vec<(i64, i64)>, max: i64) -> bool {
    for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let mut current_pos = pos;
        loop {
            if current_pos.0 < 0 || current_pos.0 > max || current_pos.0 < 0 || current_pos.0 > max
            {
                return false;
            }

            if is_green(positions, current_pos) {
                break;
            }

            current_pos.0 += d.0;
            current_pos.1 += d.1;
        }
    }

    return true;
}

fn is_inside(pos: (i64, i64), positions: &Vec<(i64, i64)>, max: i64) -> bool {
    let mut intersections = 0;

    for i in 0..pos.0 {
        if is_green(positions, (i, pos.1)) {
            intersections += 1;
        }
    }

    return intersections % 2 == 1;
}

static mut COUNTER: u64 = 0;

fn is_valid(
    (x1, y1): (i64, i64),
    (x2, y2): (i64, i64),
    positions: &Vec<(i64, i64)>,
    max: i64,
) -> bool {
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

    return is_inside(top_left, positions, max)
        && is_inside(top_right, positions, max)
        && is_inside(bot_left, positions, max)
        && is_inside(bot_right, positions, max);
}

fn flood(s: (i64, i64), b: &mut Vec<Vec<bool>>, positions: &Vec<(i64, i64)>) {
    let mut stack = vec![s];

    while let Some(pos) = stack.pop() {
        println!("flooding {:?}", pos);
        let top = (pos.0, pos.1 + 1);
        let bot = (pos.0, pos.1 - 1);
        let left = (pos.0 - 1, pos.1);
        let right = (pos.0 + 1, pos.1);

        if !b[top.0 as usize][top.1 as usize] {
            b[top.0 as usize][top.1 as usize] = true;
            if !is_green(positions, top) {
                stack.push(top);
            } else {
                println!("is green!!");
            }
        }

        if !b[bot.0 as usize][bot.1 as usize] {
            b[bot.0 as usize][bot.1 as usize] = true;
            if !is_green(positions, bot) {
                stack.push(bot);
            }
        }

        if !b[left.0 as usize][left.1 as usize] {
            b[left.0 as usize][left.1 as usize] = true;
            if !is_green(positions, left) {
                stack.push(left);
            }
        }

        if !b[right.0 as usize][right.1 as usize] {
            b[right.0 as usize][right.1 as usize] = true;
            if !is_green(positions, right) {
                stack.push(right);
            }
        }
    }
}

fn main() {
    let data = fs::read_to_string("input.test.txt").expect("input file");

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

    // let t = (positions[0].0 - 1, positions[0].1 + 1); // everything else is outside
    // let t = (positions[0].0 + 1, positions[0].1 + 1); // everything else is outside (test)
    // println!("is inside: {}", is_inside(t, &positions, max));

    // let mut b: Vec<Vec<bool>> = Vec::with_capacity(max as usize);

    // for i in 0..(max as usize) {
    //     b.push(Vec::with_capacity(max as usize));
    //     for _ in 0..max {
    //         b[i].push(false);
    //     }
    // }

    // println!("{}", b.len());
    // println!("{}", b.len());
    // println!("{}", b.len());
    // println!("{}", b.len());

    // println!("flodded");

    // for y in 0..max {
    //     for x in 0..max {
    //         if positions.contains(&(x as i64, y as i64)) {
    //             print!("#");
    //         } else if b[x as usize][y as usize] {
    //             print!("X");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     print!("\n");
    // }

    let areas: Vec<Vec<i64>> = positions
        .iter()
        .map(|p1| positions.iter().map(|p2| area(*p1, *p2)).collect())
        .collect();

    println!(
        "areas: {}",
        areas.iter().flatten().collect::<Vec<&i64>>().len(),
    );
    println!("max: {max}");

    let allowed_areas: Vec<Vec<i64>> = areas
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, _a)| is_valid(positions[*x], positions[y], &positions, max))
                .map(|(_x, a)| *a)
                .collect()
        })
        .collect();

    let max_area = allowed_areas
        .iter()
        .flatten()
        .max()
        .expect("some max value");

    println!("{max_area}");

    for y in 0..max {
        for x in 0..max {
            if is_inside((x as i64, y as i64), &positions, max) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }

    // println!("{:?}", areas);
    // println!("{:?}", allowed_areas);
}
