macro_rules! add {
    () => {0};
    ($($a:expr), *) => {
        0 $(+$a)*
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_add() {
        assert_eq!(0, add![]);
        assert_eq!(9, add![2,3,4]);
        assert_eq!(30, add![4,5,6,7,8]);
        vec![];
    }
}