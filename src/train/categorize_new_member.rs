// TODO: 迭代器写法

enum Grade {
    Open,
    Senior,
}

pub fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    for (i, j) in data.iter() {
        match open_or_senior_item(*i, *j) {
            Grade::Open => v.push(String::from("Open")),
            Grade::Senior => v.push(String::from("Senior")),
        }
    }
    v
}

fn open_or_senior_item(age: i32, handicap: i32) -> Grade {
    if age >= 55 && handicap > 7 {
        Grade::Senior
    }
    else {
        Grade::Open
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(open_or_senior(vec![(45, 12), (55,21), (19, -2), (104, 20)]), vec!["Open", "Senior", "Open", "Senior"]);
        assert_eq!(open_or_senior(vec![(3, 12), (55,1), (91, -2), (54, 23)]), vec!["Open", "Open", "Open", "Open"]);
    }
}