use std::collections::{HashMap};
use std::hash::Hash;

#[derive(Debug, Clone)]
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

impl<T> IntoIterator for Counter<T>
    where T: Eq + Hash
{
    type Item = (T, u128);
    type IntoIter = std::collections::hash_map::IntoIter<T, u128>;

    fn into_iter(self) -> Self::IntoIter {
        self.counts.into_iter()
    }
}

impl<T> FromIterator<T> for Counter<T> 
    where T: Eq + Hash
{
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut c = Self::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}

impl<T> FromIterator<(T, u128)> for Counter<T> 
    where T: Eq + Hash
{
    fn from_iter<I: IntoIterator<Item=(T, u128)>>(iter: I) -> Self {
        let mut c = Self::new();

        for (i, count) in iter {
            c.add_count(i, count);
        }
        
        c
    }
}