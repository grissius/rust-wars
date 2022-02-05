fn solution(s: &str) -> Vec<String> {
  (0..s.len()).step_by(2).filter_map(|i| match (s.chars().nth(i), s.chars().nth(i+1)) {
    (Some(a), b) => {
      let mut pair = a.to_string();
      pair.push(match b { Some(x) => x, None => '_' });
      Some(pair)
    },
    _ => None
  }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution(""), [] as [&str; 0]);
    }
}
