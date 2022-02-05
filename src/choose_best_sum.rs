use std::{iter::Enumerate, slice::Iter};

struct Result {
    sum: i32,
    sol: i32,
}

fn choose_best_sum_r_(val: i32, iter: &Enumerate<Iter<i32>>, depth: i32, solution_indices: Vec<usize>, max: i32) -> i32 {
    let remaining_branches = iter.to_owned().filter(|(i, _)| !solution_indices.contains(i)).collect::<Vec<(usize, &i32)>>();
    if depth == 0 || remaining_branches.len() == 0 {
        println!("ret {}", val);
        return val
    }
    let r = remaining_branches.iter().map(|(i, v)| {
        let mut next_sol = solution_indices.to_vec();
        next_sol.push(*i);
        println!("dive v={}, d={}, val={} {:?}", v, depth, val, next_sol);
        choose_best_sum_r_(val + *v, &iter, depth -1, next_sol, max)
    }).filter(|s| *s <= max).max().unwrap_or(val);
    println!("gather r={}, d={}", r, depth);
    r
}

fn choose_best_sum_r(value: i32, iter: &Enumerate<Iter<i32>>, max_depth: i32, solution_indices: Vec<usize>, max: i32, min: i32) -> Option<i32> {
    if solution_indices.len() >= iter.len() || value + min * (max_depth - solution_indices.len() as i32) > max {
        return None
    }
    if solution_indices.len() as i32 == max_depth {
        return Some(value)
    }
    iter.to_owned().filter_map(|(i, v)| {
        if solution_indices.contains(&i) || value + *v > max {
            return None
        }
        let mut next_sol = solution_indices.clone();
        next_sol.push(i);
        Some(choose_best_sum_r( value + *v, &iter, max_depth, next_sol, max, min))
    }).max().unwrap_or(None)
}

fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
    match choose_best_sum_r( 0, &ls.iter().enumerate(), k, vec![], t, *ls.iter().min().unwrap_or(&1)) {
        Some(x) => x,
        None => -1
    }
}

fn testing(t: i32, k: i32, ls: &Vec<i32>, exp: i32) -> () {
    assert_eq!(choose_best_sum(t, k, ls), exp)
}

#[test]
fn basics_choose_best_sum() {
    let ts = &vec![50, 55, 56, 57, 58];
    testing(163, 3, ts, 163);
    let ts = &vec![50];
    testing(163, 3, ts, -1);
    let ts = &vec![91, 74, 73, 85, 73, 81, 87];
    testing(230, 3, ts, 228);
    testing(331, 2, ts, 178);
    
}