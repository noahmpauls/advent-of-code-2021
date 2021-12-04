use std::collections::{HashSet,HashMap};
use std::hash::Hash;

pub struct Counter<T> 
    where T: Eq + Hash
{
    counts: HashMap<T, u32>,
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

    pub fn count(&self, item: T) -> Option<u32> {
        self.counts.get(&item).copied()
    }
}

pub fn find_oxygen(vals: &Vec<u16>, bits: usize) -> u32 {
    let mut remaining: HashSet<u16> = vals.iter().copied().collect();

    for i in (0..bits).rev() {
        let mut counter = Counter::new();
        let mask = 1 << i;
        let (mut zeroes, mut ones) = (HashSet::new(), HashSet::new());
        for num in remaining {
            if num & mask == 0 {
                counter.add(0);
                zeroes.insert(num);
            } else {
                counter.add(1);
                ones.insert(num);
            }
        }

        if counter.count(1) >= counter.count(0) {
            remaining = ones;
        } else {
            remaining = zeroes;
        }

        if remaining.len() == 1 {
            return (*remaining.iter().next().unwrap()).into();
        }
    }

    panic!("no oxygen value found");
}

pub fn find_co2(vals: &Vec<u16>, bits: usize) -> u32 {
    let mut remaining: HashSet<u16> = vals.iter().copied().collect();

    for i in (0..bits).rev() {
        let mut counter = Counter::new();
        let mask = 1 << i;
        let (mut zeroes, mut ones) = (HashSet::new(), HashSet::new());
        for num in remaining {
            if num & mask == 0 {
                counter.add(0);
                zeroes.insert(num);
            } else {
                counter.add(1);
                ones.insert(num);
            }
        }

        if counter.count(1) >= counter.count(0) {
            remaining = zeroes;
        } else {
            remaining = ones;
        }

        if remaining.len() == 1 {
            return (*remaining.iter().next().unwrap()).into();
        }
    }

    panic!("no co2 value found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
