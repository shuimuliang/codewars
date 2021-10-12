pub fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    match k {
        _k if _k == 0 => String::new(),
        _k => {
            match strarr.len() {
                _len if _len == 0 => String::new(),
                _len if _k > _len => String::new(),
                _len => {
                    // 构建window字符串和长度
                    let tuples: Vec<(&[&str], usize)> = strarr.windows(_k).map(
                        |words| {
                            (words, words.iter().map(|word| word.len()).sum::<usize>())
                        }).collect();
                    // 最长的长度
                    let longest_length: usize = tuples.iter().max_by_key(|x| x.1).unwrap().1;
                    // 出现最长长度的第一个位置
                    let position: usize = tuples.iter().position(|x| x.1 == longest_length).unwrap();
                    // 拼接字符串
                    tuples[position].0.join("")
                }
            }
        }
    }
}

#[test]
fn basics_longest_consec() {
    fn testing(strarr: Vec<&str>, k: usize, exp: &str) -> () {
        assert_eq!(&longest_consec(strarr, k), exp)
    }

    testing(vec!["tree", "foling", "trashy", "blue", "abcdef", "uvwxyz"], 2, "folingtrashy");
    testing(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2, "abigailtheta");
    testing(vec!["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", "oocccffuucccjjjkkkjyyyeehh"], 1,
            "oocccffuucccjjjkkkjyyyeehh");
    testing(vec![], 3, "");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 3, "ixoyx3452zzzzzzzzzzzz");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
}

// fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
//     if k == 0 {
//         return "".into()
//     }
//
//     strarr.windows(k).enumerate().map(|(idx, w)| {
//         let len : usize = w.iter().map(|l| l.len()).sum();
//         let joined = w.join("");
//         (len, -(idx as isize), joined)
//     }).max().map(|t| t.2.into()).unwrap_or("".into())
// }
// fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
//     let mut result = String::new();
//
//     if k > 0 && strarr.len() >= k {
//         for index in 0..strarr.len() - k + 1 {
//             let s: String = strarr[index..index + k].join("");
//             if s.len() > result.len() {
//                 result = s;
//             }
//         }
//     }
//
//     result
// }