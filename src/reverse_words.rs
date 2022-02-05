fn reverse_words(str: &str) -> String {
    let mut word: String = "".to_string();
    let mut acc: String = "".to_string();
    for char in str.chars() {
        if char == ' ' {
            acc.push_str(word.chars().rev().collect::<String>().as_str());
            word.truncate(0);
            acc.push(char)
        } else {
            word.push(char)
        }
    }
    acc.push_str(word.chars().rev().collect::<String>().as_str());
    acc
}

#[test]
fn test_reverse_words() {
    assert_eq!(
        reverse_words("The quick brown fox jumps over the lazy dog."),
        "ehT kciuq nworb xof spmuj revo eht yzal .god"
    );
    assert_eq!(reverse_words("apple"), "elppa");
    assert_eq!(reverse_words("a b c d"), "a b c d");
    assert_eq!(
        reverse_words("double  spaced  words"),
        "elbuod  decaps  sdrow"
    );
}