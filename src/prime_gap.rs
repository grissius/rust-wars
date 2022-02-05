fn is_prime(x: u64) -> bool {
    for t in 2..((x as f64).sqrt().ceil() as u64 + 1) {
        if x % t == 0 && t != x {
            return false;
        }
    }
    return x >= 2;
}

fn gap(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    let mut last_p = None;
    for x in m..(n+1) {
        if is_prime(x) {
            match last_p {
                Some(last) => {
                    if x - last == g as u64 {
                        return Some((last, x))
                    }
                },
                _ => {}
            }
            last_p = Some(x)
        }
    }
    return None
}

fn testing(g: i32, m: u64, n: u64, exp: Option<(u64, u64)>) -> () {
    assert_eq!(gap(g, m, n), exp)
}

#[test]
fn basics_gap() {
    testing(2, 100, 110, Some((101, 103)));
    testing(4, 100, 110, Some((103, 107)));
    testing(6, 100, 110, None);
    testing(8, 300, 400, Some((359, 367)));
}
