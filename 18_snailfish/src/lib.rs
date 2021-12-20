mod tokenize;

use tokenize::{Token, Tokenizer};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SnailNum {
    // regulars, in order
    regulars: Vec<u128>,
    // consecutive brackets of the same type adjacent to regulars;
    // n consecutive left brackets adjacent to a number gives +n, while
    // n consecutive right brackets adjacent to a number gives -n
    brackets: Vec<i32>,
}

impl SnailNum {
    pub fn parse(string: &str) -> SnailNum {
        let tokens = Tokenizer::new(string);

        let mut regulars = Vec::new();
        let mut brackets = Vec::new();

        let mut delta = 0;
        let mut last_token = None;

        for token in tokens {
            match token.clone() {
                Token::Comma => continue,
                Token::BracketLeft => {
                    if let Some(Token::BracketRight) = last_token {
                        brackets.push(delta);
                        delta = 0;
                    }
                    delta += 1;
                },
                Token::BracketRight => {
                    delta -= 1;
                }
                Token::Int(value) => {
                    let value = value.parse::<u128>().unwrap();
                    regulars.push(value);
                    if let Some(token) = last_token {
                        match token {
                            Token::BracketLeft | Token::BracketRight => {
                                brackets.push(delta);
                                delta = 0;
                            },
                            _ => (),
                        }
                    }
                }
            }
            last_token = Some(token);
        }
        brackets.push(delta);

        let result = SnailNum { regulars, brackets };
        result.check_rep();

        result
    }

    fn check_rep(&self) {
        assert_eq!(0, self.brackets.iter().sum::<i32>());
        assert_eq!(self.brackets.len(), self.regulars.len());
    }

    pub fn reduce(&mut self) {
        'outer: loop {
            // try explode
            let mut depth = self.brackets[0];
            for i in 0..self.regulars.len() {
                if
                    depth >= 5 && 
                    i + 1 < self.regulars.len() &&
                    self.brackets[i].signum() == 1 && self.brackets[i + 1].signum() == -1
                {
                    self.explode(i);
                    continue 'outer;
                } else {
                    if self.brackets[i].signum() == -1 {
                        depth += self.brackets[i];
                    }
                    if i+1 < self.regulars.len() {
                        if self.brackets[i+1].signum() == 1 {
                            depth += self.brackets[i+1];
                        }
                    }
                }
            }

            // try split
            for i in 0..self.regulars.len() {
                if self.regulars[i] > 9 {
                    self.split(i);
                    continue 'outer;
                }
            }

            break 'outer;
        }

        self.check_rep();
    }

    fn explode(&mut self, i: usize) {
        // validate
        assert!(i + 1 < self.regulars.len());
        assert!(self.brackets[i].signum() == 1 && self.brackets[i + 1].signum() == -1);

        // explode
        let left = self.regulars[i];
        let right = self.regulars[i + 1];
        if i > 0 {
            self.regulars[i - 1] += left;
        }
        if i + 2 < self.regulars.len() {
            self.regulars[i + 2] += right;
        }

        self.regulars[i] = 0;
        self.regulars.remove(i + 1);

        self.brackets[i] -= 1;
        self.brackets[i + 1] += 1;

        if self.brackets[i] == 0 {
            self.brackets.remove(i);
        } else if self.brackets[i + 1] == 0 {
            self.brackets.remove(i + 1);
        }

        self.check_rep();
    }

    fn split(&mut self, i: usize) {
        // validate
        assert!(self.regulars[i] > 9);
        assert!(i < self.regulars.len());

        // split
        let value = self.regulars[i];
        let left = value / 2;
        let right = (value / 2) + if value % 2 != 0 { 1 } else { 0 };

        self.regulars[i] = left;
        self.regulars.insert(i + 1, right);

        if self.brackets[i].signum() == 1 {
            self.brackets[i] += 1;
            self.brackets.insert(i + 1, -1)
        } else {
            self.brackets[i] -= 1;
            self.brackets.insert(i, 1);
        }
        
        self.check_rep();
    }

    pub fn magnitude(&self) -> u128 {
        self.check_rep();

        let mut i = 0;
        let mut magnitudes = self.regulars.clone();
        let mut brackets = self.brackets.clone();

        while magnitudes.len() > 1 {
            if i + 1 >= magnitudes.len() {
                i -= 1;
            } else if brackets[i + 1].signum() == 1 {
                i += 1;
            } else if brackets[i].signum() == 1 {
                magnitudes[i] = (3 * magnitudes[i]) + (2 * magnitudes[i + 1]);
                magnitudes.remove(i + 1);

                brackets[i] -= 1;
                brackets[i + 1] += 1;
                if brackets[i] == 0 {
                    brackets.remove(i);
                } else if brackets[i + 1] == 0 {
                    brackets.remove(i + 1);
                }
            } else {
                i -= 1;
            }
        }

        magnitudes[0]
    }
}

use std::ops::Add;

impl Add for SnailNum {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let regulars: Vec<u128> = self.regulars.into_iter().chain(other.regulars.into_iter()).collect();
        let mut brackets: Vec<i32> = self.brackets.into_iter().chain(other.brackets.into_iter()).collect();
        brackets[0] += 1;
        let last = brackets.len() - 1;
        brackets[last] -= 1;

        let mut result = SnailNum { regulars, brackets };
        result.reduce();

        result
    }
}

use std::fmt;

impl fmt::Display for SnailNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        for i in 0..self.regulars.len() {
            if self.brackets[i].signum() == 1 {
                result.push_str(&"[".repeat(self.brackets[i].try_into().unwrap()));
            } 
            result.push_str(&format!("{}", self.regulars[i]));
            if self.brackets[i].signum() == -1 {
                result.push_str(&"]".repeat(self.brackets[i].abs().try_into().unwrap()));
            }
            if i + 1 < self.regulars.len() {
                result.push(',');
            }
        }

        write!(f, "{}", result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse(to_parse: &str) {
        let snail = SnailNum::parse(to_parse);
        assert_eq!(to_parse, &format!("{}", snail));
    }

    #[test]
    fn test_parse_1() {
        test_parse("[1,[2,3]]");
    }

    #[test]
    fn test_parse_2() {
        test_parse("[[1,2],[[3,4],5]]");
    }

    #[test]
    fn test_parse_3() {
        test_parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_parse_4() {
        test_parse("[[[[1,1],[2,2]],[3,3]],[4,4]]");
    }

    #[test]
    fn test_parse_5() {
        test_parse("[[[[3,0],[5,3]],[4,4]],[5,5]]");
    }

    #[test]
    fn test_parse_6() {
        test_parse("[[[[5,0],[7,4]],[5,5]],[6,6]]");
    }

    #[test]
    fn test_parse_7() {
        test_parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    }

    #[test]
    fn test_reduce_1() {
        let mut snail = SnailNum::parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        snail.reduce();
        let expected = SnailNum::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(expected, snail);
    }

    #[test]
    fn test_reduce_2() {
        let mut snail = SnailNum::parse("[0,[0,[0,[20,0]]]]");
        snail.reduce();

        /*
        [0,[0,[0,[20,0]]]]
        [0,[0,[0,[[10,10],0]]]]
        [0,[0,[10,[0,10]]]]
        [0,[0,[[5,5],[0,[5,5]]]]]
        [0,[0,[[5,5],[5,0]]]]
        */

        let expected = SnailNum::parse("[0,[0,[[5,5],[5,0]]]]");

        assert_eq!(expected, snail);
    }

    #[test]
    fn test_add_1() {
        let left = SnailNum::parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let right = SnailNum::parse("[1,1]");

        let sum = left + right;
        let expected = SnailNum::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(expected, sum);
    }

    #[test]
    fn test_add_2() {
        let left = SnailNum::parse("[[1,2],[3,4]]");
        let right = SnailNum::parse("[[5,6],[7,8]]");

        let sum = left + right;
        let expected = SnailNum::parse("[[[1,2],[3,4]],[[5,6],[7,8]]]");
        assert_eq!(expected, sum);
    }

    #[test]
    fn test_add_3() {
        let left = SnailNum::parse("[6,7]");
        let right = SnailNum::parse("[[[[1,2],3],4],5]");

        let sum = left + right;
        let expected = SnailNum::parse("[[6,8],[[[0,5],4],5]]");
        assert_eq!(expected, sum);
    }

    #[test]
    fn test_add_4() {
        let left = SnailNum::parse("[6,9]");
        let right = SnailNum::parse("[[[[1,2],3],4],5]");

        let sum = left + right;
        let expected = SnailNum::parse("[[6,[5,5]],[[[0,5],4],5]]");
        assert_eq!(expected, sum);
    }

    #[test]
    fn test_add_5() {
        let left = SnailNum::parse("[1,[2,[3,[4,5]]]]");
        let right = SnailNum::parse("[[[[5,4],3],2],1]");

        let sum = left + right;
        let expected = SnailNum::parse("[[1,[8,[0,6]]],[[[5,7],2],1]]");
        assert_eq!(expected, sum);
    }

    #[test]
    fn test_add_6() {
        let left = SnailNum::parse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
        let right = SnailNum::parse("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");

        let sum = left + right;
        let expected = SnailNum::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
        assert_eq!(expected, sum);
    }

    #[test]
    fn test_add_sequence_1() {
        let sum = vec![
            "[1,1]",
            "[2,2]",
            "[3,3]",
            "[4,4]",
        ].into_iter().map(|num| SnailNum::parse(num)).reduce(|a, b| a + b).unwrap();

        let expected = SnailNum::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]");

        assert_eq!(expected, sum);
    }

    #[test]
    fn test_add_sequence_2() {
        let sum = vec![
            "[1,1]",
            "[2,2]",
            "[3,3]",
            "[4,4]",
            "[5,5]",
        ].into_iter().map(|num| SnailNum::parse(num)).reduce(|a, b| a + b).unwrap();

        let expected = SnailNum::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]");

        assert_eq!(expected, sum);
    }

    #[test]
    fn test_add_sequence_3() {
        let sum = vec![
            "[1,1]",
            "[2,2]",
            "[3,3]",
            "[4,4]",
            "[5,5]",
            "[6,6]",
        ].into_iter().map(|num| SnailNum::parse(num)).reduce(|a, b| a + b).unwrap();

        let expected = SnailNum::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]");

        assert_eq!(expected, sum);
    }

    #[test]
    fn test_add_sequence_big() {
        let mut snails = vec![
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ].into_iter().map(|num| SnailNum::parse(num));

        let intermediates: Vec<_> = vec![
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
            "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
            "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
            "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
            "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
            "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
            "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        ].into_iter().map(|num| SnailNum::parse(num)).collect();

        let mut sum = snails.next().unwrap();

        for (i, snail) in snails.enumerate() {
            sum = sum + snail;
            assert_eq!(intermediates[i], sum, "failed at intermeidate {}", i);
        }

        let expected_mag = 3488;
        let actual_mag = sum.magnitude();

        assert_eq!(expected_mag, actual_mag);
    }

    #[test]
    fn test_magnitude_simple() {
        let mag = SnailNum::parse("[9,1]").magnitude();
        let expected = 29;
        assert_eq!(expected, mag);
    }

    #[test]
    fn test_magnitude_nested() {
        let mag = SnailNum::parse("[[9,1],[1,9]]").magnitude();
        let expected = 129;
        assert_eq!(expected, mag);
    }

    #[test]
    fn test_magnitude_1() {
        let mag = SnailNum::parse("[[1,2],[[3,4],5]]").magnitude();
        let expected = 143;
        assert_eq!(expected, mag);
    }

    #[test]
    fn test_magnitude_2() {
        let mag = SnailNum::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude();
        let expected = 1384;
        assert_eq!(expected, mag);
    }

    #[test]
    fn test_magnitude_3() {
        let mag = SnailNum::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude();
        let expected = 445;
        assert_eq!(expected, mag);
    }

    #[test]
    fn test_magnitude_4() {
        let mag = SnailNum::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude();
        let expected = 791;
        assert_eq!(expected, mag);
    }

    #[test]
    fn test_magnitude_5() {
        let mag = SnailNum::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude();
        let expected = 1137;
        assert_eq!(expected, mag);
    }

    #[test]
    fn test_magnitude_6() {
        let mag = SnailNum::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude();
        let expected = 3488;
        assert_eq!(expected, mag);
    }
}
