pub fn dig_pow(n: i64, p: i32) -> i64 {
    let mut result: i64 = 0;
    let digit_s = n.to_string();
    let digit_len = n.to_string().len();

    for i in 0..digit_len {
        let numberser = digit_s.chars().nth(i).unwrap().to_digit(10).unwrap();
        result += numberser.pow(p as u32 + i as u32) as i64;
    }

    let x = n.pow(p as u32) as i64;
    if result == x {
        return p as i64;
    } else if result % n == 0 {
        return result / n;
    } else {
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i64, p: i32, exp: i64) -> () {
        println!(" n: {:?};", n);
        println!("p: {:?};", p);
        let ans = dig_pow(n, p);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(89, 1, 1);
        dotest(92, 1, -1);
        dotest(46288, 3, 51);
    }
}