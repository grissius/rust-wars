fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    let mut top = 0;
    let mut bottom = matrix.len().saturating_sub(1);
    let mut left = 0;
    let mut right = match matrix.first() { Some(x) => x.len().saturating_sub(1), _ => 0 };
    let mut res = vec![];
    while bottom >= top && right >= left {
        match matrix.iter().nth(top) {
            Some(top) => res.append(&mut top[left..right].to_vec()),
            _ => {},
        }
        res.append(&mut matrix.iter().skip(top).take(bottom - top + 1).filter_map(|x| match x.iter().nth(right) { Some(x) => Some(*x), _ => None }).collect::<Vec<i32>>());
        match matrix.iter().nth(bottom) {
            Some(bottom) => res.append(&mut bottom[left..right].iter().rev().map(|x| *x).collect::<Vec<i32>>()),
            _ => {},
        }
        res.append(&mut matrix.iter().skip(top + 1).take(bottom.saturating_sub(top + 1)).rev().filter_map(|x| match x.iter().nth(left) { Some(x) => Some(*x), _ => None }).collect::<Vec<i32>>());
        top += 1;
        left += 1;
        bottom = bottom.saturating_sub(1);
        right = right.saturating_sub(1);

    }
    return res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test1() {
        let square = &[
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9],
        ];
        let expected = vec![1,2,3,6,9,8,7,4,5];
        assert_eq!(snail(square), expected);
    }
    
    #[test]
    fn sample_test2() {
        let square = &[
            vec![1,2,3],
            vec![8,9,4],
            vec![7,6,5],
        ];
        let expected = vec![1,2,3,4,5,6,7,8,9];
        assert_eq!(snail(square), expected);
    }
    
    #[test]
    fn sample_test3() {
        let square: &[Vec<i32>; 1] = &[Vec::new()];
        let expected = Vec::new();
        assert_eq!(snail(square), expected, "Failed with empty input");
    }
    
    #[test]
    fn sample_test4() {
        let square = &[vec![1]];
        let expected = vec![1];
        assert_eq!(snail(square), expected);
    }
}
