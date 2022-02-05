type Factors = Vec<(i64, i32)>;

pub fn factors(input: i64) -> Factors {
    let mut rest = input;
    let mut result = vec![];
    for f in 2..=((rest as f64).sqrt().ceil() as i64) {
        let mut times = 0;
        while rest % f == 0 {
            rest = rest / f;
            times += 1;
        }
        if times > 0 { result.push((f, times)) }
    }
    if rest != 1 { result.push((rest, 1)) }
    result
}

fn format_factors(factors: Factors) -> String {
    factors.iter().map(|pair| match pair {
        (f, 1) => format!("({})", f),
        (f, c) => format!("({}**{})", f, c),
    }).collect::<String>()
}

fn prime_factors(input: i64) -> String {
    format_factors(factors(input))
}

#[test]
fn test_factors() {
    println!("{:?}", prime_factors(4))
}