use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, VecDeque},
    fs,
};

#[derive(PartialEq, Eq)]
struct Node {
    state: Vec<u64>,
    path: Vec<usize>,
    g: u64, // cost
    f: u64, // cost + heuristic
}

// Reverse ordering for min-heap behavior
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f).then_with(|| other.g.cmp(&self.g))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// approx distance left
fn heuristic(state: &Vec<u64>, goal: &Vec<u64>) -> u64 {
    state
        .iter()
        .zip(goal)
        .map(|(s, g)| g.saturating_sub(*s))
        .max()
        .expect("some heuristic value")
}

fn a_star(buttons: &Vec<Vec<usize>>, goal: &Vec<u64>) -> Vec<usize> {
    let start_state: Vec<u64> = vec![0; goal.len()];

    let mut open: BinaryHeap<Node> = BinaryHeap::new();
    let mut best_g: HashMap<Vec<u64>, u64> = HashMap::new();

    let h0 = heuristic(&start_state, goal);

    open.push(Node {
        state: start_state.clone(),
        path: vec![],
        g: 0,
        f: h0,
    });

    best_g.insert(start_state, 0);

    while let Some(node) = open.pop() {
        let state = &node.state;

        if state == goal {
            return node.path;
        }

        println!(
            "searching at depth {}, stack is {} long",
            node.g,
            open.len()
        );
        // println!("stack: {:?}", stack);

        // 1 + 1;

        for (button_index, button) in buttons.iter().enumerate() {
            let mut new_state = state.clone();

            for index in button {
                new_state[*index] += 1;

                // skip if missed
                if new_state[*index] > goal[*index] {
                    continue;
                }
            }

            let new_g = node.g + 1;

            if let Some(&known_g) = best_g.get(&new_state) {
                // we already have a better path to that state
                if new_g >= known_g {
                    continue;
                }
            }

            let h = heuristic(&new_state, goal);
            let new_f = new_g + h;

            let mut new_path = node.path.clone();
            new_path.push(button_index);

            best_g.insert(new_state.clone(), new_g);

            open.push(Node {
                state: new_state,
                path: new_path,
                g: new_g,
                f: new_f,
            });
        }
    }

    // didn't find a way
    return vec![];
}
fn bfs(buttons: &Vec<Vec<usize>>, goal: &Vec<u64>) -> Vec<usize> {
    let mut state_stack: VecDeque<Vec<u64>> = VecDeque::from(vec![(vec![0; goal.len()])]);
    let mut path_stack: VecDeque<Vec<usize>> = VecDeque::from(vec![(vec![])]);

    while let Some(state) = state_stack.pop_front() {
        let path = path_stack.pop_front().expect("a path for each state");

        // println!(
        //     "searching at depth {}, stack is {} long",
        //     path.len(),
        //     state_stack.len()
        // );
        // println!("stack: {:?}", stack);

        // 1 + 1;

        for (button_index, button) in buttons.iter().enumerate() {
            let mut new_state = state.clone();
            for index in button {
                new_state[*index] += 1;
            }

            for (index, value) in new_state.iter().enumerate() {
                if *value > goal[index] {
                    continue;
                }
            }

            if state_stack.contains(&new_state) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(button_index);

            if new_state == *goal {
                return new_path;
            }

            state_stack.push_back(new_state);
            path_stack.push_back(new_path);
        }
    }

    // didn't find a way
    return vec![];
}

fn main() {
    let data = fs::read_to_string("input.test.txt").expect("an input file");

    let table_data: Vec<Vec<&str>> = data.split('\n').map(|l| l.split(' ').collect()).collect();

    let lights_goals: Vec<Vec<bool>> = table_data
        .iter()
        .map(|l| {
            l.iter()
                .next()
                .expect("lights def")
                .chars()
                .filter_map(|c| {
                    if "[]".contains(c) {
                        None
                    } else {
                        if c == '#' { Some(true) } else { Some(false) }
                    }
                })
                .collect()
        })
        .collect();

    let buttons: Vec<Vec<Vec<usize>>> = table_data
        .iter()
        .map(|l| {
            l[1..l.len() - 1]
                .iter()
                .map(|b| b.chars().filter(|c| !"()".contains(*c)).collect())
                .map(|s: String| {
                    s.split(",")
                        .map(|n| n.parse().expect("a number"))
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .collect();

    let joltages: Vec<Vec<u64>> = table_data
        .iter()
        .map(|l| {
            l[l.len() - 1]
                .chars()
                .filter(|c| !"{}".contains(*c))
                .collect::<String>()
                .split(",")
                .map(|n| n.parse().expect("a number"))
                .collect()
        })
        .collect();

    // println!("{:?}", lights_goals);
    // println!("{:?}", buttons);
    // println!("{:?}", joltages);

    let mut sum = 0;
    for i in 0..lights_goals.len() {
        let path: Vec<usize> = a_star(&buttons[i], &joltages[i]);
        println!(
            "path is of length {}: {:?} for buttons {:?} and goal {:?}",
            path.len(),
            path,
            buttons[i],
            joltages[i]
        );
        sum += path.len();
    }
    println!("sum is {sum}");
}

// part 1
// fn bfs(buttons: &Vec<Vec<usize>>, goal: &Vec<bool>) -> Vec<usize> {
//     let mut stack: VecDeque<(Vec<bool>, Vec<usize>)> =
//         VecDeque::from(vec![(vec![false; goal.len()], vec![])]);

//     while let Some((state, path)) = stack.pop_front() {
//         // println!("searching at depth {}", path.len());
//         // println!("stack: {:?}", stack);

//         // 1 + 1;

//         for (button_index, button) in buttons.iter().enumerate() {
//             let mut new_state = state.clone();
//             for index in button {
//                 new_state[*index] = !new_state[*index];
//             }

//             let mut new_path = path.clone();
//             new_path.push(button_index);

//             if new_state == *goal {
//                 return new_path;
//             }

//             stack.push_back((new_state, new_path));
//         }
//     }

//     // didn't find a way
//     return vec![];
// }

// fn main() {
//     let data = fs::read_to_string("input.txt").expect("an input file");

//     let table_data: Vec<Vec<&str>> = data.split('\n').map(|l| l.split(' ').collect()).collect();

//     let lights_goals: Vec<Vec<bool>> = table_data
//         .iter()
//         .map(|l| {
//             l.iter()
//                 .next()
//                 .expect("lights def")
//                 .chars()
//                 .filter_map(|c| {
//                     if "[]".contains(c) {
//                         None
//                     } else {
//                         if c == '#' { Some(true) } else { Some(false) }
//                     }
//                 })
//                 .collect()
//         })
//         .collect();

//     let buttons: Vec<Vec<Vec<usize>>> = table_data
//         .iter()
//         .map(|l| {
//             l[1..l.len() - 1]
//                 .iter()
//                 .map(|b| b.chars().filter(|c| !"()".contains(*c)).collect())
//                 .map(|s: String| {
//                     s.split(",")
//                         .map(|n| n.parse().expect("a number"))
//                         .collect::<Vec<usize>>()
//                 })
//                 .collect::<Vec<Vec<usize>>>()
//         })
//         .collect();

//     println!("{:?}", lights_goals);
//     println!("{:?}", buttons);

//     let mut sum = 0;
//     for i in 0..lights_goals.len() {
//         let path: Vec<usize> = bfs(&buttons[i], &lights_goals[i]);
//         println!(
//             "path is {:?} for buttons {:?} and goal {:?}",
//             path, buttons[i], lights_goals[i]
//         );
//         sum += path.len();
//     }
//     println!("sum is {sum}");
// }
