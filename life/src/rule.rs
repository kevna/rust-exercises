use std::collections::{HashMap, HashSet};

pub struct Rule {
    birth: HashSet<u32>,
    survival: HashSet<u32>,
}

impl Rule {
    pub fn new(rulestring: &str) -> Rule {
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

    #[rstest]
    #[case(
        "B3/S23",
        Rule{
            birth: vec![3].into_iter().collect(),
            survival: vec![2, 3].into_iter().collect(),
        }
    )]
    #[case(
        "B45678/S2345",
        Rule{
            birth: vec![4, 5, 6, 7, 8].into_iter().collect(),
            survival: vec![2, 3, 4, 5].into_iter().collect(),
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
        assert_eq!(expected, actual)
    }
}
