use std::ops::Index;

const KESY: &'static [&'static str] = &["Jabroni", "School Counselor", "Programmer", "Bike Gang Member", "Politician", "Rapper"];
const VALUES: &'static [&'static str] = &["Patron Tequila", "Anything with Alcohol", "Hipster Craft Beer", "Moonshine", "Your tax dollars", "Cristal"];
const NO_MATCH: &'static str = "Beer";

// "Jabroni"	"Patron Tequila"
// "School Counselor"	"Anything with Alcohol"
// "Programmer"	"Hipster Craft Beer"
// "Bike Gang Member"	"Moonshine"
// "Politician"	"Your tax dollars"
// "Rapper"	"Cristal"
// anything else	"Beer"

pub fn get_drink_by_profession(param: &str) -> &'static str {
    let index = KESY.iter().position(
        |&word| word.to_lowercase() == param.to_lowercase()
    );
    match index {
       Some(pos) => VALUES.index(pos),
        None => NO_MATCH
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(get_drink_by_profession("jabrOni"), "Patron Tequila", "'Jabroni' should map to 'Patron Tequila'");
        assert_eq!(get_drink_by_profession("scHOOl counselor"), "Anything with Alcohol", "'School Counselor' should map to 'Anything with alcohol'");
        assert_eq!(get_drink_by_profession("prOgramMer"), "Hipster Craft Beer", "'Programmer' should map to 'Hipster Craft Beer'");
        assert_eq!(get_drink_by_profession("bike ganG member"), "Moonshine", "'Bike Gang Member' should map to 'Moonshine'");
        assert_eq!(get_drink_by_profession("poLiTiCian"), "Your tax dollars", "'Politician' should map to 'Your tax dollars'");
        assert_eq!(get_drink_by_profession("rapper"), "Cristal", "'Rapper' should map to 'Cristal'");
        assert_eq!(get_drink_by_profession("pundit"), "Beer", "'Pundit' should map to 'Beer'");
        assert_eq!(get_drink_by_profession("Pug"), "Beer", "'Pug' should map to 'Beer'");
    }
}
