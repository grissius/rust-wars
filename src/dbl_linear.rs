use std::{
    collections::BTreeSet,
};
// https://www.codewars.com/kata/5672682212c8ecf83e000050/train/rust
fn dbl_linear(n: u32) -> u32 {
    let mut index = n as usize;
    let mut acc = BTreeSet::new();
    acc.insert(1);
    let mut newbies: Vec<u32> = vec![1];
    let mut stable = false;
    while acc.len() <= index || !stable {
        let stable_val = newbies.iter().min().expect("no min") * 2 + 1;
        stable = if let Some(x) = acc.iter().nth(index) { *x < stable_val } else { false };
        newbies = {
            let xs = newbies.iter();
            xs.flat_map(|f| vec![2 * f + 1, f.saturating_mul(3).saturating_add(1)].into_iter())
                .collect()
        };
        for n in &newbies {
            acc.insert(*n);
        }
    }
    *acc.iter().nth(index).expect("fucked")
}

#[cfg(test)]
mod tests {
    use super::dbl_linear;
    fn testing(n: u32, exp: u32) -> () {
        assert_eq!(dbl_linear(n), exp)
    }

    #[test]
    fn basics_dbl_linear() {
        // println!("{}", dbl_linear(2000)); // 19773
        // println!("{}", dbl_linear(10000)); // 80914
        // println!("{}", dbl_linear(100000)); // 80914
        testing(10, 22);
        testing(20, 57);
        testing(30, 91);
        testing(50, 175);
        testing(100, 447);
    }
}
// u = [1, 3, 4, 7, 9, 10, 13, 15, 19, 21, 22, 27, ...]
