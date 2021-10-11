/// Given a string of words, you need to find the highest scoring word.
///
/// Each letter of a word scores points according to its position in the alphabet: a = 1, b = 2, c = 3 etc.
///
/// You need to return the highest scoring word as a string.
///
/// If two words score the same, return the word that appears earliest in the original string.
///
/// All letters will be lowercase and all inputs will be valid.
/// input.split_ascii_whitespace().rev().max_by_key(|s| s.chars().map(|c| c as u16 - 96).sum::<u16>()).unwrap_or("")

pub fn high(input: &str) -> &str {
    let words: Vec<&str> = input
        .split(' ').
        collect();
    let scores: Vec<u32> = input.
        split(' ').
        map(|x| {
        x.chars().map(|c| 1 + c as u32 - 'a' as u32).sum() })
        .collect();
    let mut tuples: Vec<(&&str, &u32)> = words.iter().zip(scores.iter()).collect();

    tuples.sort_by(|a, b| b.1.cmp(&a.1));
    *(tuples.first().unwrap().0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        high("man i need a taxi up to ubud");
        assert_eq!(high("man i need a taxi up to ubud"), "taxi");
        assert_eq!(high("what time are we climbing up the volcano"), "volcano");
        assert_eq!(high("take me to semynak"), "semynak");
        assert_eq!(high("massage yes massage yes massage"), "massage");
        assert_eq!(high("take two bintang and a dance please"), "bintang");
        assert_eq!(high("aa b"), "aa");
        assert_eq!(high("b aa"), "b");
        assert_eq!(high("bb d"), "bb");
        assert_eq!(high("d bb"), "d");
        assert_eq!(high("aaa b"), "aaa");
    }
}