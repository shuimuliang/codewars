// fn xo(string: &'static str) -> bool {
//     string.chars().fold(0, |a, c|{
//         match c {
//             'x' | 'X' => a + 1,
//             'o' | 'O' => a - 1,
//             _ => a
//         }
//     }) == 0
// }

pub fn xo(string: &'static str) -> bool {
    let lower_string: String = string.to_lowercase();
    let c1 = lower_string.chars().filter(|c| *c == 'x').count();
    let c2 = lower_string.chars().filter(|c| *c == 'o').count();
    c1 == c2
}

#[test]
fn returns_expected() {
    assert_eq!(xo("xo"), true);
    assert_eq!(xo("Xo"), true);
    assert_eq!(xo("xxOo"), true);
    assert_eq!(xo("xxxm"), false);
    assert_eq!(xo("Oo"), false);
    assert_eq!(xo("ooom"), false);
}