mod solution {
    use std::collections::BTreeSet;

    pub fn range_extraction(a: &[i32]) -> String {
        let mut tree = BTreeSet::new();
        for n in a {
            tree.insert(*n);
        }
        let mut output = Vec::new();
        let mut interval = (None, None);
        let mut consume_interval = |x: (&i32, &i32)| {
            let (start, last) = x;        println!("{:?}", &tree);

            if start == last {
                output.push(format!("{}", &start));
            } else if  ((start - last) as i32).abs() == 1 {
                output.push(format!("{}", &start));
                output.push(format!("{}", &last));

            } else {
                output.push(format!("{}-{}", &start, last));
            }
        };
        for v in tree.iter() {
            match interval {
                (Some(start), Some(last)) if ((v - last) as i32).abs() == 1 => {
                    interval = (Some(start), Some(v))
                }
                (Some(start), Some(last)) => {
                    consume_interval((start, last));
                    interval = (Some(v), Some(v));
                }
                _ => interval = (Some(v), Some(v)),
            }
        }
        if let (Some(start), Some(last)) = interval {
            consume_interval((start, last));
        }
        output.join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            "-6,-3-1,3-5,7-11,14,15,17-20",
            solution::range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
            ])
        );
        assert_eq!(
            "-3--1,2,10,15,16,18-20",
            solution::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20])
        );
    }
}
