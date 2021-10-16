pub fn duplicate_encode(word: &str) -> String {
    let lower_word = word.to_lowercase();
    lower_word.chars().map(|c| {
        match lower_word.matches(c).count() {
            1 => '(',
            _ => ')',
        }
    }
    ).collect()
}

#[test]
fn run_tests() {
    assert_eq!(duplicate_encode("din"), "(((");
    assert_eq!(duplicate_encode("recede"), "()()()");
    assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
    assert_eq!(duplicate_encode("(( @"), "))((");
}