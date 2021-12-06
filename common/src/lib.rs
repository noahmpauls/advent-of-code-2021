use std::collections::{HashMap};
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

    pub fn with_count_ge(&self, min: u32) -> Vec<&T> {
        self.counts.iter().filter_map(|(key, value)| {
            if *value >= min {
                Some(key)
            } else {
                None
            }
        }).collect()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
