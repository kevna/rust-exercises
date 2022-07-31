use std::collections::{HashMap, HashSet};
use phf::{Map, phf_map};

static NAMED_RULES: Map<&'static str, &'static str> = phf_map! {
    "seeds" => "B2",
    "live-free-or-die" => "B2/S0",
    "life-without-death" => "B3/S012345678",
    "flock" => "B3/S12",
    "mazectric" => "B3/S1234",
    "maze" => "B3/S12345",
    "original" => "B3/S23",
    "highlife" => "B36/S23",
    "move" => "B368/S245",
    "coagulation" => "B378/S235678",
    "walled-cities" => "B45678/S2345",
    "bacteria" => "B34/S456",
    "longlife" => "B345/S5",
    "amoeba" => "B357/S1358",
};


pub struct Rule {
    birth: HashSet<u32>,
    survival: HashSet<u32>,
}

impl Rule {
    pub fn new(rulestring: &str) -> Rule {
        let mut rulestring = rulestring;
        if NAMED_RULES.contains_key(rulestring) {
            rulestring = NAMED_RULES[rulestring];
        }
        const RADIX: u32 = 10;
        let mut accumulator: HashMap<&str,  HashSet<u32>> = HashMap::new();
        for rule in rulestring.split("/") {
            let key = &rule[..1];
            let mut values = HashSet::new();
            for ch in rule[1..].chars() {
                values.insert(ch.to_digit(RADIX).unwrap());
            }
            accumulator.insert(key, values);
        }
        return Rule{
            birth: accumulator["B"].clone(),
            survival: accumulator["S"].clone(),
        };
    }

    pub fn apply(&self, alive: &bool, neigbours: &u32) -> bool {
        return (*alive && self.survival.contains(neigbours)) || (!*alive && self.birth.contains(neigbours));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    macro_rules! set {
        ( $( $x:expr ),* ) => {{ // Match zero or more comma delimited items
            let mut temp_set = HashSet::new();
            $( temp_set.insert($x); )* // Do this for eatch matched item
            temp_set
        }};
    }

    fn classic() -> Rule {
        Rule{
            birth: set![3],
            survival: set![2, 3],
        }
    }

    #[rstest]
    #[case("B3/S23", classic())]
    #[case("original", classic())] // lookup a named rule
    #[case(
        "B45678/S2345",
        Rule{
            birth: set![4, 5, 6, 7, 8],
            survival: set![2, 3, 4, 5],
        }
    )]
    #[case(
        "B2/S0000",
        Rule{
            birth: set![2],
            survival: set![0],
        }
    )]
    fn test_rule_new(#[case] rulestring: &str, #[case] expected: Rule) {
        let rule = Rule::new(rulestring);
        assert_eq!(expected.birth, rule.birth);
        assert_eq!(expected.survival, rule.survival);
    }

    #[rstest]
    #[case(false, 2, false)]
    #[case(false, 3, true)]
    #[case(false, 4, false)]
    #[case(true, 1, false)]
    #[case(true, 2, true)]
    #[case(true, 3, true)]
    #[case(true, 4, false)]
    fn test_rule_apply(#[case] alive: bool, #[case] neigbours: u32, #[case] expected: bool) {
        let rule = Rule::new("B3/S23");
        let actual = rule.apply(&alive, &neigbours);
        assert_eq!(expected, actual);
    }
}
