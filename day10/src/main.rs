use std::fs;

use good_lp::*;

// find vec x so that Ax = b, maximize c^Tx and x>0
// b is goal
// A is which button increments the counter
// x is the number of times a button has been pressed
// c is 1 (or -1)
fn solve_simplex(buttons: &Vec<Vec<usize>>, goal: &Vec<u64>) -> f64 {
    let mut A: Vec<Vec<u32>> = vec![vec![0; buttons.len()]; goal.len()]; // width of buttons, height of goal, u32 is probably fine?
    for (x, button) in buttons.iter().enumerate() {
        for y in button {
            A[*y][x] = 1;
        }
    }

    let mut vars = variables!();
    let xs: Vec<Variable> = buttons
        .iter()
        .map(|_| vars.add(variable().integer().min(0)))
        .collect();

    let mut problem = vars
        .minimise(xs.iter().fold(Expression::from(0), |acc, x| acc + x))
        .using(default_solver);

    for (row, &bi) in A.iter().zip(goal) {
        let expr: Expression = row
            .iter()
            .zip(&xs)
            .fold(Expression::from(0), |acc, (a, x)| acc + *x * *a);
        problem = problem.with(expr.eq(bi as u32)); // hmm u32? probably fine
    }

    let answer = problem.solve().expect("a solution");

    let solution: Vec<f64> = xs.iter().map(|x| answer.value(*x)).collect();
    println!("{:?}", solution);

    let sum = solution.iter().fold(0.0, |acc, s| acc + s);
    sum
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

    let mut sum = 0.0;
    for i in 0..lights_goals.len() {
        // let i = 9;
        let n = solve_simplex(&buttons[i], &joltages[i]);
        // println!(
        //     "path is of length {}: for buttons {:?} and goal {:?}",
        //     n, buttons[i], joltages[i]
        // );
        sum += n;
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
