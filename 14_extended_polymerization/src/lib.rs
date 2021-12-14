use std::collections::HashMap;
use common::{char_windows};

pub struct PolyRule {
    pair: String,
    out: char,
    polymer: String,
}

impl PolyRule {
    pub fn new(pair: &str, out: char) -> PolyRule {
        assert_eq!(2, pair.len());

        let mut pair_iter = pair.chars();
        let polymer = [pair_iter.next().unwrap(), out, pair_iter.next().unwrap()].into_iter().collect();
        PolyRule {
            pair: String::from(pair),
            out,
            polymer: polymer,
        }
    }

    pub fn pair(&self) -> &str {
        &self.pair
    }

    pub fn out(&self) -> char {
        self.out
    }

    pub fn polymer(&self) -> &str {
        &self.polymer
    }
}

pub struct PolyRuleSet {
    rules: HashMap<String, String>,
}

impl PolyRuleSet {
    pub fn from(rules: Vec<PolyRule>) -> PolyRuleSet {
        let rules = rules.into_iter().map(|rule| {
            (String::from(rule.pair()), String::from(rule.polymer()))
        }).collect();
        PolyRuleSet{ rules }
    }

    pub fn polymer(&self, pair: &str) -> Option<&str> {
        if let Some(out) = self.rules.get(pair) {
            Some(&out)
        } else {
            None
        }
    }

    pub fn polymer_pairs(&self, pair: &str) -> Option<[&str; 2]> {
        if let Some(out) = self.rules.get(pair) {
            let mut pairs = char_windows(out, 2);
            Some([pairs.next().unwrap(), pairs.next().unwrap()])
        } else {
            None
        }
    }

    pub fn polymerize(&self, template: &str) -> String {
        template.chars().take(1).chain(
            char_windows(template, 2)
                .flat_map(|pair| {
                    self.polymer(pair).unwrap().chars().skip(1)
                })
        ).collect()
    }
}

use std::hash::Hash;

pub struct Counter<T> 
    where T: Eq + Hash
{
    counts: HashMap<T, u128>,
}

impl<T> Counter<T> 
    where T: Eq + Hash
{
    pub fn new() -> Counter<T> {
        Counter {
            counts: HashMap::new(),
        }
    }

    pub fn add(&mut self, item: T) {
        let entry = self.counts.entry(item).or_insert(0);
        *entry += 1;
    }

    pub fn add_count(&mut self, item: T, count: u128) {
        let entry = self.counts.entry(item).or_insert(0);
        *entry += count;
    }

    pub fn count(&self, item: T) -> Option<u128> {
        self.counts.get(&item).copied()
    }

    pub fn keys(&self) -> impl Iterator<Item = &T> {
        self.counts.keys()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, &u128)> {
        self.counts.iter()
    }

    pub fn with_count_ge(&self, min: u128) -> Vec<&T> {
        self.counts.iter().filter_map(|(key, value)| {
            if *value >= min {
                Some(key)
            } else {
                None
            }
        }).collect()
    }

    pub fn most_frequent(&self) -> Option<&T> {
        self.counts.iter().fold((None, 0), |(max_key, max_count), (key, value)| {
            if *value > max_count {
                (Some(key), *value)
            } else {
                (max_key, max_count)
            }
        }).0
    }

    pub fn least_frequent(&self) -> Option<&T> {
        self.counts.iter().fold((None, u128::MAX), |(min_key, min_count), (key, value)| {
            if *value < min_count {
                (Some(key), *value)
            } else {
                (min_key, min_count)
            }
        }).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ruleset() -> PolyRuleSet {
        let rules = [
            ("AB", 'C'),
            ("DE", 'F'),
            ("FE", 'G'),
            ("AC", 'H'),
            ("CB", 'I'),
            ("BD", 'J'),
            ("DF", 'K'),
            ("BJ", 'L'),
            ("JD", 'M'),
        ].into_iter()
            .map(|r| PolyRule::new(r.0, r.1))
            .collect();

        PolyRuleSet::from(rules)
    }

    #[test]
    fn it_works() {
        let rules = make_ruleset();

        let input = "ABDE";
        let actual = rules.polymerize(&input);
        let expected = String::from("ACBJDFE");
        assert_eq!(expected, actual);

        let input = actual;
        let actual = rules.polymerize(&input);
        let expected = String::from("AHCIBLJMDKFGE");
        assert_eq!(expected, actual);
    }
}
