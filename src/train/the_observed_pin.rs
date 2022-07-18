use std::collections::HashMap;

// https://zhuanlan.zhihu.com/p/421808973
// build hashmap, method 3
fn adj_keys(ch: char) -> &'static str {
    match ch {
        '1' => "124",
        '2' => "1235",
        '3' => "236",
        '4' => "1457",
        '5' => "24568",
        '6' => "3569",
        '7' => "478",
        '8' => "05789",
        '9' => "689",
        '0' => "08",
        _ => "",
    }
}

// build hashmap, method 4
const ADJACENT_MAP: [&'static [char]; 10] = [
    &['0', '8'],
    &['1', '2', '4'],
    &['2', '1', '3', '5'],
    &['3', '2', '6'],
    &['4', '1', '5', '7'],
    &['5', '2', '4', '6', '8'],
    &['6', '3', '5', '9'],
    &['7', '4', '8'],
    &['8', '5', '7', '9', '0'],
    &['9', '6', '8']
];

fn permutation(adjacent_array: &[&str]) -> Vec<String> {
    let mut permutations = Vec::new();

    if let Some((head, tail_array)) = adjacent_array.split_first() {
        if tail_array.is_empty() {
            // head.chars().map(|ch| ch.to_string()).collect::<Vec<String>>()
            head.chars().for_each(|ch| permutations.push(ch.to_string()));
        } else {
            // head.chars + tail.iter
            for ch in head.chars() {
                let tail_permutations = permutation(tail_array);
                for tail_permutation in tail_permutations {
                    permutations.push(format!("{}{}", ch, tail_permutation));
                }
            }
        }
    }
    permutations
}

fn get_pins(observed: &str) -> Vec<String> {
    // build hashmap, method 1
    let map = HashMap::from(
        [('1', "124"), ('2', "1235"), ('3', "236"), ('4', "1457"), ('5', "24568"),
            ('6', "3569"), ('7', "478"), ('8', "05789"), ('9', "689"), ('0', "08")]);

    // build hashmap, method 2
    // let keys :[char;10]= ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    // let adjacent_nums:[&'static str;10]= ["124", "1235", "236", "1457", "24568", "3569", "478", "05789", "689", "08"];
    // let map = keys.iter().zip(adjacent_nums.iter()).collect::<HashMap<_, _>>();

    let adjacent_array = observed.chars().map(|c| *map.get(&c).unwrap()).collect::<Vec<&str>>();
    permutation(&adjacent_array)
}

#[cfg(test)]
mod tests {
    use super::get_pins;
    use itertools::Itertools;

    #[test]
    fn sample_foo() {
        assert_eq!(get_pins("8").iter().sorted().collect::<Vec<&String>>(),
                   vec!["0", "5", "7", "8", "9"]);
        assert_eq!(get_pins("11").iter().sorted().collect::<Vec<&String>>(),
                   vec!["11", "12", "14", "21", "22", "24", "41", "42", "44"]);
        assert_eq!(get_pins("369").iter().sorted().collect::<Vec<&String>>(),
                   vec!["236", "238", "239", "256", "258", "259", "266", "268", "269", "296", "298", "299",
                        "336", "338", "339", "356", "358", "359", "366", "368", "369", "396", "398", "399",
                        "636", "638", "639", "656", "658", "659", "666", "668", "669", "696", "698", "699"]);
    }
}
