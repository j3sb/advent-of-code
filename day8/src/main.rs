use std::{collections::HashSet, fs};

fn dist((x1, y1, z1): &(isize, isize, isize), (x2, y2, z2): &(isize, isize, isize)) -> isize {
    let dx = x1 - x2;
    let dy = y1 - y2;
    let dz = z1 - z2;
    return dx * dx + dy * dy + dz * dz;
}

fn get_circuits(circuits: &mut Vec<HashSet<usize>>) {
    let mut modified = false;
    let mut i = 0;
    while i < circuits.len() {
        let mut j = i + 1;
        while j < circuits.len() {
            if circuits[i].iter().any(|p| circuits[j].contains(p)) {
                // put both together
                let to_merge = circuits.remove(j);
                circuits[i].extend(to_merge);
                modified = true;
            } else {
                j += 1;
            }
        }
        i += 1;
    }

    if modified {
        get_circuits(circuits);
    }
}

fn main() {
    let data = fs::read_to_string("./input.txt").expect("an input file");
    let num_pairs = 1000;

    let points: Vec<(isize, isize, isize)> = data
        .split("\n")
        .map(|s| {
            let mut nums = s.split(",").map(|v| v.parse::<isize>().expect("a isize"));

            let a = nums.next().expect("missing first number");
            let b = nums.next().expect("missing second number");
            let c = nums.next().expect("missing third number");

            (a, b, c)
        })
        .collect();

    let mut dists: Vec<(isize, usize, usize)> = vec![];
    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().skip(i + 1).enumerate() {
            let d = dist(p1, p2);
            dists.push((d, i, j + i + 1));
        }
    }

    dists.sort_by_key(|&(d, _, _)| d);

    // println!("{:?}", dists);

    let mut connections: Vec<HashSet<usize>> = dists
        .iter()
        .take(num_pairs)
        .map(|(_, i, j)| vec![*i, *j].into_iter().collect::<HashSet<usize>>())
        .collect();

    println!("{:?}", connections);
    // let connections_vec = connections.iter().map(|p| p.to_vec()).collect();
    get_circuits(&mut connections);

    println!("{:?}", connections);

    let mut lengths: Vec<usize> = connections.iter().map(|c| c.len()).collect::<Vec<usize>>();
    lengths.sort_by_key(|&l| std::cmp::Reverse(l));

    println!("{:?}", lengths);

    let res = lengths.iter().take(3).fold(1, |acc, l| acc * l);
    println!("{res}");
}
