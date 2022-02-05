use itertools::Itertools;

fn next_bigger_number(n: i64) -> i64 {
    let str = n.to_string();
    str
        .split("")
        .filter(|x| *x != "")
        .permutations(str.len())
        .map(|xs| xs.join("").parse::<i64>().expect("parse fail"))
        .filter(|x| x > &n)
        .min().unwrap_or(-1)
}>

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(21, next_bigger_number(12));
        assert_eq!(531, next_bigger_number(513));
        assert_eq!(2071, next_bigger_number(2017));
        assert_eq!(441, next_bigger_number(414));
        assert_eq!(414, next_bigger_number(144));
    }
}
