use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

fn dfs(start: &str, end: &str, map: HashMap<&str, Vec<&str>>) -> usize {
    let mut stack: VecDeque<(&str, Vec<&str>)> = VecDeque::from(vec![(start, vec![])]);

    let mut pathes_to_end: HashMap<&str, Vec<Vec<&str>>> = HashMap::new();

    let default: Vec<&str> = vec![];

    let mut num_paths: usize = 0;
    while let Some((current, path)) = stack.pop_front() {
        println!("{current}");
        if current == end {
            println!("{:?}", path);

            1 + 1;

            for (index, device) in path.iter().enumerate() {
                let mut new_path = path[index..path.len()].to_vec();
                new_path.push(current);

                pathes_to_end.entry(device).or_insert(vec![]).push(new_path);
            }

            if path.contains(&"dac") && path.contains(&"fft") {
                num_paths += 1;
            }
            continue;
        }

        if let Some(pathes) = pathes_to_end.get(current) {
            println!("found {current} in pathes, pathes: {:?}", pathes);

            num_paths += pathes.len();
            continue;
        }

        let maybe_childs = map.get(current);
        if maybe_childs == None {
            println!("{current} has no childs");
        }
        let childs = maybe_childs.unwrap_or(&default);

        for child in childs {
            let mut new_path = path.clone();
            new_path.push(current);
            stack.push_back((child, new_path));
        }
    }

    // println!("{:#?}", pathes_to_end);

    num_paths
}

// assumption: there are no loops bc else there would be infinite number of paths in some cases. eg: you: you out
fn main() {
    let data = fs::read_to_string("input.test2.txt").expect("an input file");

    let map: HashMap<&str, Vec<&str>> = data
        .split("\n")
        .map(|l| {
            let mut l_it = l.split(":");
            let source = l_it.next().expect("a source device");
            let outputs: Vec<&str> = l_it
                .next()
                .expect("output devices")
                .split(" ")
                .skip(1)
                .collect();
            (source, outputs)
        })
        .collect();

    println!("{:#?}", map);

    let n = dfs("svr", "out", map);

    println!("{n}");
}
