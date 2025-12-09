#[allow(unused_imports)]
use std::{fs, vec};

fn in_range(i: usize, l: usize, h: usize) -> bool {
    if l <= i && i <= h {
        return true;
    }

    return false;
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("read a file");

    let split_data: Vec<&str> = data.split("\n\n").collect();

    let ranges_string = split_data[0];

    let ranges: Vec<(usize, usize)> = ranges_string
        .split("\n")
        .map(|s| {
            let a: Vec<_> = s.split("-").collect();
            (
                a[0].parse().expect("a usize"),
                a[1].parse().expect("a usize"),
            )
        })
        .collect();

    let mut count: usize = 0;
    let mut counted: Vec<(usize, usize)> = vec![];

    for (lower_iter, higher_iter) in ranges {
        let mut lower = lower_iter;
        let mut higher = higher_iter;

        counted = counted
            .iter()
            .filter(|(l, h)| {
                if in_range(*l, lower, higher) && in_range(*h, lower, higher) {
                    count -= h - l + 1;
                    println!("removing {}", h - l + 1);
                    return false;
                }
                return true;
            })
            .copied()
            .collect();

        let mut ok;
        loop {
            ok = true;
            for (l, h) in &counted {
                if in_range(lower, *l, *h) {
                    lower = h + 1;
                    ok = false;
                }
                if in_range(higher, *l, *h) {
                    higher = l - 1;
                    ok = false;
                }
            }
            if higher < lower {
                break;
            }
            if ok {
                break;
            }
        }

        if lower <= higher {
            count += higher - lower + 1;
            counted.push((lower, higher));
        } else {
            println!("{lower}, {higher}");
        }
    }

    println!("{:?}", counted);
    println!("{count}");
}

// part one
// fn main() {
//     let data = fs::read_to_string("input.txt").expect("read a file");

//     let split_data: Vec<&str> = data.split("\n\n").collect();

//     let ranges_string = split_data[0];
//     let id_string = split_data[1];

//     let ranges: Vec<(usize, usize)> = ranges_string
//         .split("\n")
//         .map(|s| {
//             let a: Vec<_> = s.split("-").collect();
//             (
//                 a[0].parse().expect("a usize"),
//                 a[1].parse().expect("a usize"),
//             )
//         })
//         .collect();

//     let ids: Vec<usize> = id_string
//         .split("\n")
//         .map(|s| s.parse().expect("a usize"))
//         .collect();

//     let mut count = 0;
//     for id in ids {
//         for (lower, higher) in &ranges {
//             if *lower <= id && id <= *higher {
//                 count += 1;
//                 println!("{id} is valid");
//                 break;
//             }
//         }
//     }

//     println!("{count}");
// }
