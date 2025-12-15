use std::{collections::VecDeque, fs};

fn bfs(buttons: &Vec<Vec<usize>>, goal: &Vec<bool>) -> Vec<usize> {
    let mut stack: VecDeque<(Vec<bool>, Vec<usize>)> =
        VecDeque::from(vec![(vec![false; goal.len()], vec![])]);

    while let Some((state, path)) = stack.pop_front() {
        // println!("searching at depth {}", path.len());
        // println!("stack: {:?}", stack);

        // 1 + 1;

        for (button_index, button) in buttons.iter().enumerate() {
            let mut new_state = state.clone();
            for index in button {
                new_state[*index] = !new_state[*index];
            }

            let mut new_path = path.clone();
            new_path.push(button_index);

            if new_state == *goal {
                return new_path;
            }

            stack.push_back((new_state, new_path));
        }
    }

    // didn't find a way
    return vec![];
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("an input file");

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

    println!("{:?}", lights_goals);
    println!("{:?}", buttons);

    let mut sum = 0;
    for i in 0..lights_goals.len() {
        let path: Vec<usize> = bfs(&buttons[i], &lights_goals[i]);
        println!(
            "path is {:?} for buttons {:?} and goal {:?}",
            path, buttons[i], lights_goals[i]
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
