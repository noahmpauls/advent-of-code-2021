use std::collections::{HashMap, HashSet};

pub struct SegmentDecoder {
    nums_to_wires: HashMap<u8, UnknownDigit>,
    wires_to_nums: HashMap<String, u8>,
}

impl SegmentDecoder {
    pub fn from(digits: &Vec<&str>) -> SegmentDecoder {
        let mut seg_counts: HashMap<usize, HashSet<UnknownDigit>> = HashMap::new();
        digits.iter()
            .map(|d| UnknownDigit::new(d))
            .for_each(|digit| {
                let entry = seg_counts.entry(digit.len()).or_insert(HashSet::new());
                entry.insert(digit);
            });
        
        let mut nums_to_wires: HashMap<u8, UnknownDigit> = HashMap::new();

        for (num, size) in [(1, 2), (4, 4), (7, 3), (8, 7)] {
            let digit = seg_counts.remove(&size).unwrap();
            assert!(digit.len() == 1);
            nums_to_wires.insert(num, digit.into_iter().next().unwrap());
        }

        // oof...

        // "4" produces 2 segment overlaps with "2"
        let two = seg_counts.get(&5).unwrap().iter().filter(|d| d.overlap(nums_to_wires.get(&4).unwrap().clone()) == 2).next().unwrap().clone();
        seg_counts.get_mut(&5).unwrap().remove(&two);
        // "1" produces 2 segment overlaps with "3"
        let three = seg_counts.get(&5).unwrap().iter().filter(|d| d.overlap(nums_to_wires.get(&1).unwrap().clone()) == 2).next().unwrap().clone();
        seg_counts.get_mut(&5).unwrap().remove(&three);
        // "5" remains
        let five = seg_counts.get(&5).unwrap().iter().next().unwrap().clone();

        // "1" produces 1 segment overlap with "6"
        let six = seg_counts.get(&6).unwrap().iter().filter(|d| d.overlap(nums_to_wires.get(&1).unwrap().clone()) == 1).next().unwrap().clone();
        seg_counts.get_mut(&6).unwrap().remove(&six);
        // "4" produces 4 segment overlaps with "9"
        let nine = seg_counts.get(&6).unwrap().iter().filter(|d| d.overlap(nums_to_wires.get(&4).unwrap().clone()) == 4).next().unwrap().clone();
        seg_counts.get_mut(&6).unwrap().remove(&nine);
        // "0" remains
        let zero = seg_counts.get(&6).unwrap().iter().next().unwrap().clone();

        for (num, digit) in [
            (2, two),
            (3, three),
            (5, five),
            (6, six),
            (9, nine),
            (0, zero),
        ] {
            nums_to_wires.insert(num, digit);
        }

        let wires_to_nums = nums_to_wires.iter().map(|(k, v)| (v.chars.iter().collect(), *k)).collect();

        SegmentDecoder { nums_to_wires, wires_to_nums }
    }

    pub fn decode(&self, digit: &str) -> Option<u8> {
        let mut sorted: Vec<char> = digit.chars().collect();
        sorted.sort();
        let sorted: String = sorted.iter().collect();
        self.wires_to_nums.get(&sorted).copied()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct UnknownDigit {
    // sorted list of chars in digit
    chars: Vec<char>,
}

impl UnknownDigit {
    fn new(digit: &str) -> UnknownDigit {
        let mut chars: Vec<char> = digit.chars().collect();
        chars.sort();
        UnknownDigit { chars }
    }

    fn len(&self) -> usize {
        self.chars.len()
    }

    // count of segments both digits contain
    fn overlap(&self, other: UnknownDigit) -> usize {
        let this: HashSet<_> = self.chars.iter().collect();
        other.chars.iter().filter(|c| this.contains(c)).count()
    }
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
