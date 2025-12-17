use std::{collections::HashMap, fs};

fn dfs<'a>(
    start: &'a str,
    end: &'a str,
    solved: &mut HashMap<&'a str, (u64, u64, u64, u64)>,
    map: &HashMap<&'a str, Vec<&'a str>>,
) -> (u64, u64, u64, u64) {
    // (#with both dac and fft, only dac, only fft, none)

    if let Some(values) = solved.get(start) {
        println!("cache hit on {start}");
        return *values;
    }

    let default: Vec<&str> = vec![];

    if start == end {
        return (0, 0, 0, 1);
    }

    let maybe_childs = map.get(start);
    if maybe_childs == None {
        println!("{start} has no childs");
        // panic!("{start} has no childs");
    }
    let childs = maybe_childs.unwrap_or(&default);

    let mut n = (0, 0, 0, 0);
    for child in childs {
        let d = dfs(&child, end, solved, map);

        if start == "fft" {
            // add both to both
            n.0 += d.0;
            // add only dac to both
            n.0 += d.1;
            // none to fft
            n.2 += d.3;
            // none is zero
        } else if start == "dac" {
            // add both to both
            n.0 += d.0;
            // add only fft to both
            n.0 += d.2;
            // none to dac
            n.1 += d.3;
            // none is zero
        } else {
            n.0 += d.0;
            n.1 += d.1;
            n.2 += d.2;
            n.3 += d.3;
        }
    }

    solved.insert(start, n);

    return n;
}

// assumption: there are no loops bc else there would be infinite number of paths in some cases. eg: you: you out
fn main() {
    let data = fs::read_to_string("input.txt").expect("an input file");

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

    // println!("{:#?}", map);

    // dac to out ~ 5k paths
    // dac to fft 0 paths
    // fft to dac ~ 9 million paths
    let solved = &mut HashMap::<&str, (u64, u64, u64, u64)>::new();
    // let dac_out = dfs("dac", "out", solved, &map);
    // println!("dac to out: {}", dac_out.len());
    // let fft_dac = dfs("fft", "dac", solved, &map);
    // println!("fft to dac: {}", fft_dac.len());

    let n = dfs("svr", "out", solved, &map);

    println!("{:?}", n);

    // let sols = n
    //     .iter()
    //     .filter(|path| path.contains(&"dac") && path.contains(&"fft"))
    //     .collect::<Vec<&Vec<&str>>>();

    // println!("{:?}", sols.len());
}
