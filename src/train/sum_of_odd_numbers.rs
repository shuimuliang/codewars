fn n_sum(n: i64) -> i64 {
    n.pow(2)
}

fn n_count(n: i64) -> i64 {
    n * (n + 1) / 2
}

pub fn row_sum_odd_numbers(n: i64) -> i64 {
    match n {
        1 => 1,
        _ => n_sum(n_count(n)) - n_sum(n_count(n - 1))
    }
}

#[test]
fn returns_expected() {
    assert_eq!(row_sum_odd_numbers(1), 1);
    assert_eq!(row_sum_odd_numbers(42), 74088);
}