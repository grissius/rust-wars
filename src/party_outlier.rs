fn find_outlier(xs: &[i32]) -> i32 {
  let rem = if xs.iter().take(3).filter(|x| *x % 2 == 0).count() <= 1 { 0 } else { 1 };
  match xs.iter().find(|x| (*x).rem_euclid(2) == rem) {
    Some(x) => *x,
    _ => 0
  }
}

fn find_outlier2(xs: &[i32]) -> i32 {
  let mut one = (0, 0);
  let mut zero = (0, 0);
  for x in xs {
    if x % 2 == 0 {
      zero.0 = *x;
      zero.1 += 1;
    } else {
      one.0 = *x;
      one.1 += 1;
    }
  }
  if zero.1 == 1 { zero.0 } else { one.0 }
}

#[test]
fn basic() {
    assert_eq!(find_outlier(&[2, 4, 0, 100, 4, 11, 2602, 36]), 11);
    assert_eq!(find_outlier(&[160, 3, 1719, 19, 11, 13, -21]), 160);
    assert_eq!(find_outlier(&[-160, -3, -1719, -19, -11, -13, 21]), -160);
    assert_eq!(find_outlier2(&[-160, -3, -1719, -19, -11, -13, 21]), -160);
}