fn valid_braces(s: &str) -> bool {
    let mut stack = vec![];
    for ch in s.chars() {
        match (stack.last(), ch) {
            (Some('('), ')') | (Some('{'), '}') | (Some('['), ']') => {
                stack.pop();
            }
            _ => {
                stack.push(ch);
            }
        }
    }
    stack.is_empty()
}

#[test]
fn test_valid_braces() {
    assert_eq!(valid_braces("()"), true);
    assert_eq!(valid_braces("[(]"), false);
    assert_eq!(valid_braces("())"), false);
}
