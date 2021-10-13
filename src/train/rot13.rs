/// ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz
/// NOPQRSTUVWXYZABCDEFGHIJKLMnopqrstuvwxyzabcdefghijklm

pub fn rot13(message: &str) -> String {
    let rot13: u8 = 13;
    let a1_end_pos: u8 = b'z';
    let a2_end_pos: u8 = b'Z';
    let interval: u8 = 26;

    message.chars().map(|c|

        match c {
            'a'..='z'
            => {
                let offset = c as u8 + rot13;
                if offset > a1_end_pos {
                    (offset - interval) as char
                } else {
                    offset as char
                }
            }
            'A'..='Z' => {
                let offset = c as u8 + rot13;
                if offset > a2_end_pos {
                    (offset - interval) as char
                } else {
                    offset as char
                }
            }
            c => {
                c as char
            }
        }
    ).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(rot13("Avoid success at all costs!"), "Nibvq fhpprff ng nyy pbfgf!");
        assert_eq!(rot13("test"), "grfg");
        assert_eq!(rot13("Test"), "Grfg");
        assert_eq!(rot13("1325!"), "1325!");

        assert_eq!(rot13("Codewars"), "Pbqrjnef");
        assert_eq!(rot13("Avoid success at all costs!"), "Nibvq fhpprff ng nyy pbfgf!");
        assert_eq!(rot13("10+2 is twelve."), "10+2 vf gjryir.");
        assert_eq!(rot13("abcdefghijklmnopqrstuvwxyz"), "nopqrstuvwxyzabcdefghijklm");
        assert_eq!(rot13("1325!§"), "1325!§");
    }
}

// fn rot13(message: &str) -> String {
//     message.chars().map(|c| {
//         match c {
//             'A' ..= 'M' | 'a' ..= 'm' => ((c as u8) + 13) as char,
//             'N' ..= 'Z' | 'n' ..= 'z' => ((c as u8) - 13) as char,
//             _ => c,
//         }
//     }).collect()
// }