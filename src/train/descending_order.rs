// Descending Order
// https://www.codewars.com/kata/5467e4d82edf8bbf40000155/train/rust
// TODO: 迭代器写法

// use std::iter::FromIterator;
// fn descending_order(x: u64) -> u64 {
//     let mut result = x.to_string().chars().collect::<Vec<char>>();
//     result.sort_by(|a, b| b.cmp(a));
//     String::from_iter(result).parse::<u64>().unwrap()
// }

pub fn descending_order(x: u64) -> u64 {
    let mut vec: Vec<u64> = Vec::new();
    let mut left: u64 = x;
    let mut res: u64 = 0;

    loop {
        vec.push(left % 10);
        left = left / 10;
        if left == 0 {
            break;
        }
    }
    vec.sort_by(|a, b| b.cmp(a));

    for i in vec.iter() {
        res *= 10;
        res += i;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::descending_order;

    #[test]
    fn returns_expected() {
        assert_eq!(descending_order(0), 0);
        assert_eq!(descending_order(1), 1);
        assert_eq!(descending_order(15), 51);
        assert_eq!(descending_order(1021), 2110);
        assert_eq!(descending_order(123456789), 987654321);
        assert_eq!(descending_order(145263), 654321);
        assert_eq!(descending_order(1254859723), 9875543221);
    }
}