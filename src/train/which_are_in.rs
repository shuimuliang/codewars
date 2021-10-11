pub fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for prefix in arr_a.iter() {
        if arr_b.iter().any(|x| x.contains(prefix)) {
           res.push(  String::from(*prefix))
        }
    }
    res.sort_by(|a, b| a.partial_cmp(b).unwrap());
    res.dedup();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(in_array(
            &["xyz", "live", "strong"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ), ["live", "strong"]);

        assert_eq!(in_array(
            &["live", "strong", "arp"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ), ["arp", "live", "strong"]);

        assert_eq!(in_array(
            &["tarp", "mice", "bull"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ), [] as [&str; 0]);

        assert_eq!(in_array(
            &["live", "strong", "arp", "arp"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ), ["arp", "live", "strong"]);
    }
}
