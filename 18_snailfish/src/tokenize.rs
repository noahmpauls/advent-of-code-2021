#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token {
    BracketLeft,
    BracketRight,
    Comma,
    Int(String),
}

pub struct Tokenizer {
    string: Vec<char>,
    i: usize,
}

impl Tokenizer {
    pub fn new(string: &str) -> Tokenizer {
        Tokenizer {
            string: string.chars().collect(),
            i: 0,
        }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.string.len() {
            None
        } else {
            let token = loop {
                match self.string[self.i] {
                    '[' => break Token::BracketLeft,
                    ']' => break Token::BracketRight,
                    ',' => break Token::Comma,
                    c if c.is_whitespace() => {
                        self.i += 1;
                        if self.i >= self.string.len() {
                            return None
                        } else {
                            continue
                        }
                    },
                    c if c.is_digit(10) => {
                        let mut int = String::from(c);
                        while self.i + 1 < self.string.len() && self.string[self.i + 1].is_digit(10) {
                            self.i += 1;
                            int.push(self.string[self.i]);
                        }
                        break Token::Int(int);
                    },
                    c => panic!("invalid char {}", c),
                }
            };
            self.i += 1;
            Some(token)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_simple() {
        let string = "[8,9]";
        let tokens: Vec<Token> = Tokenizer::new(string).collect();
        let expected = vec![
            Token::BracketLeft,
            Token::Int(String::from("8")),
            Token::Comma,
            Token::Int(String::from("9")),
            Token::BracketRight,
        ];

        assert_eq!(expected, tokens);
    }

    #[test]
    fn tokenize_nested() {
        let string = "[1, [2,   3  ]]";
        let tokens: Vec<Token> = Tokenizer::new(string).collect();
        let expected = vec![
            Token::BracketLeft,
            Token::Int(String::from('1')),
            Token::Comma,
            Token::BracketLeft,
            Token::Int(String::from('2')),
            Token::Comma,
            Token::Int(String::from('3')),
            Token::BracketRight,
            Token::BracketRight,
        ];

        assert_eq!(expected, tokens);
    }
}