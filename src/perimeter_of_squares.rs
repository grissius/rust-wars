// https://www.codewars.com/kata/559a28007caad2ac4e000083/train/rust

fn fib(n: u64) -> u64 {
  let mut acc = (1, 1);
  for _i in 2..n {
    let new = acc.0 + acc.1;
    acc.0 = acc.1;
    acc.1 = new
  }
  acc.1
}

#[test]
fn test_fib() {
  assert_eq!(fib(1), 1);
  assert_eq!(fib(2), 1);
  assert_eq!(fib(3), 2);
  assert_eq!(fib(4), 3);
  assert_eq!(fib(5), 5);
}

fn acc_fib(n: u64) -> u64 {
  let mut acc = (1, 1);
  let mut sum = 2;
  for _i in 2..n {
    let new = acc.0 + acc.1;
    acc.0 = acc.1;
    acc.1 = new;
    sum += new;
  }
  sum
}

#[test]
fn acc_fib_test() {
  //assert_eq!(acc_fib(1), 1);
  assert_eq!(acc_fib(2), 2);
  assert_eq!(acc_fib(3), 4);
  assert_eq!(acc_fib(4), 7);
  assert_eq!(acc_fib(5), 12);
}


fn perimeter(n: u64) -> u64 {
  4 * acc_fib(n + 1)
}

fn dotest(n: u64, exp: u64) -> () {
  assert_eq!(perimeter(n), exp)
}

#[test]
fn basics_perimeter() {
  dotest(5, 80);
  dotest(7, 216);
  dotest(20, 114624);
  dotest(30, 14098308);
}