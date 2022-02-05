use std::collections::HashMap;

fn factors(input: i64) -> Vec<(i64, i32)> {
  let mut rest = input;
  let mut result = vec![];
  for f in 2..=((rest as f64).abs().sqrt().ceil() as i64) {
      let mut times = 0;
      // println!("{} {} {}", &f, &rest, &rest.rem_euclid(f));
      while rest.rem_euclid(f) == 0 {
        println!("{} {} {}", &rest, &f, &times);
          rest = rest / f;
          times += 1;
      }
      if times > 0 { result.push((f, times)) }
  }
  if rest.abs() != 1 { result.push((rest.abs(), 1)) }
  result
}

#[test]
fn test_factors() {
  println!("{:?}", factors(-45));
  println!("{:?}", (-45i64).rem_euclid(5));
}

fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    let mut x = l.iter().fold(HashMap::new(), |mut acc, n| {
        for f in factors(*n) {
            match acc.insert(f.0, *n) {
              Some(prev) => { acc.insert(f.0, prev + n); }
              None => {}
            };
        }
        acc
    }).into_iter().collect::<Vec<(i64, i64)>>();
    x.sort();
    println!("{:?}", &l);
    x
}

fn testing(l: Vec<i64>, exp: Vec<(i64, i64)>) -> () {
    assert_eq!(sum_of_divided(l), exp)
}

#[test]
fn basics_sum_of_divided() {
    testing(vec![12, 15], vec![(2, 12), (3, 27), (5, 15)]);
    testing(
        vec![15, 21, 24, 30, 45],
        vec![(2, 54), (3, 135), (5, 90), (7, 21)],
    );
    testing(
      vec![15, 21, 24, 30, -45],
      vec![(2, 54), (3, 45), (5, 0), (7, 21)],
  );
}
