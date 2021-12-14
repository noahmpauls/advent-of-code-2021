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

    pub fn add_count(&mut self, item: T, count: u32) {
        let entry = self.counts.entry(item).or_insert(0);
        *entry += count;
    }

    pub fn count(&self, item: T) -> Option<u32> {
        self.counts.get(&item).copied()
    }

    pub fn keys(&self) -> impl Iterator<Item = &T> {
        self.counts.keys()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, &u32)> {
        self.counts.iter()
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
        self.counts.iter().fold((None, u32::MAX), |(min_key, min_count), (key, value)| {
            if *value < min_count {
                (Some(key), *value)
            } else {
                (min_key, min_count)
            }
        }).0
    }
}

// https://stackoverflow.com/a/51261570/11898061
pub fn char_windows<'a>(src: &'a str, size: usize) -> impl Iterator<Item = &'a str> {
    src.char_indices()
        .flat_map(move |(from, _)| {
            src[from..].char_indices()
                .skip(size - 1)
                .next()
                .map(|(to, c)| {
                    &src[from .. from + to + c.len_utf8()]
                })
        })
}


#[cfg(test)]
mod tests {

    // #[test]
    // fn it_works() {

    // }
}
