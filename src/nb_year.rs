fn nb_year(p0: i32, percent: f64, aug: i32, p: i32) -> i32 {
    if p0 >= p {
        return 0;
    }
    1 + nb_year(
        p0 + (p0 as f64 * (percent / 100.)) as i32 + aug,
        percent,
        aug,
        p,
    )
}

#[test]
fn nb_year_test() {
    assert_eq!(nb_year(1500, 5., 100, 5000), 15);
    assert_eq!(nb_year(1500000, 2.5, 10000, 2000000), 10);
}