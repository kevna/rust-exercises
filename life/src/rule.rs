use std::collections::HashSet;
use phf::{Map, phf_map};
use std::str::FromStr;

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

#[derive(Debug, PartialEq)]
pub struct Rule {
    birth: HashSet<u32>,
    survival: HashSet<u32>,
}

impl FromStr for Rule {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rulestring = s;
        if NAMED_RULES.contains_key(rulestring) {
            rulestring = NAMED_RULES[rulestring];
        }
        const RADIX: u32 = 10;
        let mut birth = HashSet::new();
        let mut survival = HashSet::new();
        let mut accumulator = &mut survival;
        for ch in rulestring.chars() {
            match ch {
                'B' | '/' => { accumulator = &mut birth; }
                'S' => { accumulator = &mut survival; }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'  => {
                    accumulator.insert(ch.to_digit(RADIX).unwrap());
                }
                _ => { return Err("Unrecognised content in rulestring"); }
            }
        }
        Ok(Rule{birth, survival})
    }
}

impl Rule {
    pub fn default() -> Rule {
        "original".parse().unwrap()
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
    #[case("23/3", classic())] // legacy S/B rule notation without "B" and "S" labels
    #[case(
        "B45678/S2345",
        Rule{
            birth: set![4, 5, 6, 7, 8],
            survival: set![2, 3, 4, 5],
        }
    )]
    #[case(
        "B2", // Seeds rule has no survival section
        Rule{
            birth: set![2],
            survival: HashSet::new(),
        }
    )]
    #[case(
        "B2/S0000",
        Rule{
            birth: set![2],
            survival: set![0],
        }
    )]
    fn test_rule_from_str_ok(#[case] rulestring: &str, #[case] expected: Rule) {
        let rule = rulestring.parse();
        assert_eq!(Ok(expected), rule);
    }

    #[rstest]
    #[case("B3/S23V", "Unrecognised content in rulestring")]
    #[case("B3\\S23", "Unrecognised content in rulestring")]
    fn test_rule_from_str_err(#[case] rulestring: &str, #[case] expected: &str) {
        let rule = Rule::from_str(rulestring);
        assert_eq!(Err(expected), rule);
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
        let rule: Rule = "B3/S23".parse().unwrap();
        let actual = rule.apply(&alive, &neigbours);
        assert_eq!(expected, actual);
    }
}
