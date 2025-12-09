use std::fs;

trait Neighbors {
    fn neighbors(&self, x: usize, y: usize) -> usize;
}

impl Neighbors for Vec<Vec<bool>> {
    fn neighbors(&self, x: usize, y: usize) -> usize {
        let mut n: usize = 0;
        for dx in -1isize..=1 {
            for dy in -1isize..=1 {
                let xi = (x as isize + dx) as usize;
                let yi = (y as isize + dy) as usize;
                if !(dx == 0 && dy == 0) && xi < self.len() && yi < self[xi].len() && self[yi][xi] {
                    n += 1;
                }
            }
        }
        return n;
    }
}

fn remove_paper(data: &Vec<Vec<bool>>) -> (usize, String) {
    let mut removals: usize = 0;

    let table: String = data
        .iter()
        .enumerate()
        .map(|(y, r)| {
            r.iter()
                .enumerate()
                .map(|(x, b)| {
                    if *b {
                        if data.neighbors(x, y) < 4 {
                            removals += 1;
                            "x"
                        } else {
                            "@"
                        }
                    } else {
                        "."
                    }
                })
                .collect::<String>()
                + "\n"
        })
        .collect();

    println!("{table}");

    (removals, table.chars().take(table.len() - 1).collect())
}

fn main() {
    let data = fs::read_to_string("./input.txt").expect("read the file");

    let mut ans: usize = 0;

    let mut vec_data: Vec<Vec<bool>> = data
        .split("\n")
        .map(|s| -> Vec<_> { s.chars().collect() })
        .map(|r| r.iter().map(|c| *c == '@').collect())
        .collect();

    let (mut removals, mut new_state) = remove_paper(&vec_data);
    while removals != 0 {
        vec_data = new_state
            .split("\n")
            .map(|s| -> Vec<_> { s.chars().collect() })
            .map(|r| r.iter().map(|c| *c == '@').collect())
            .collect();
        ans += removals;
        (removals, new_state) = remove_paper(&vec_data);
    }

    println!("{ans}");
}
