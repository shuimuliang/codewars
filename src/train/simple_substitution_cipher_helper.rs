// todo:: 生命期写法

use std::collections::HashMap;

struct Cipher {
    from_map: HashMap<char, char>,
    to_map: HashMap<char, char>,
}

impl Cipher {
    fn new(map1: &str, map2: &str) -> Cipher {
        let mut from_map: HashMap<char, char> = HashMap::new();
        let mut to_map: HashMap<char, char> = HashMap::new();
        for (from, to) in map1.chars().zip(map2.chars()) {
            from_map.insert(from, to);
            to_map.insert(to, from);
        }
        Cipher {
            from_map: from_map,
            to_map: to_map,
        }
    }

    fn encode(&self, string: &str) -> String {
        string.chars().map(|c| {
            match self.from_map.get(&c) {
                Some(v) => *v,
                None => c,
            }
        }).collect::<String>()
    }

    fn decode(&self, string: &str) -> String {
        string.chars().map(|c| {
            match self.to_map.get(&c) {
                Some(v) => *v,
                None => c,
            }
        }
        ).collect::<String>()
    }
}

#[test]
fn examples() {
    let map1 = "abcdefghijklmnopqrstuvwxyz";
    let map2 = "etaoinshrdlucmfwypvbgkjqxz";

    let cipher = Cipher::new(map1, map2);

    assert_eq!(cipher.encode("abc"), "eta");
    assert_eq!(cipher.encode("xyz"), "qxz");
    assert_eq!(cipher.decode("eirfg"), "aeiou");
    assert_eq!(cipher.decode("erlang"), "aikcfu");
}