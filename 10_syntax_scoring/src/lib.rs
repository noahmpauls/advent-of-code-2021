pub fn parse_chunk(chunks: &str) -> ParseResult {
    let mut closers = Vec::new();

    for char in chunks.chars() {
        match char {
            '(' => closers.push(')'),
            '[' => closers.push(']'),
            '{' => closers.push('}'),
            '<' => closers.push('>'),
            closer => {
                if let Some(expected) = closers.pop() {
                    if expected != closer {
                        return ParseResult::Corrupted { expected, actual: closer };
                    }
                } else {
                    return ParseResult::Overflow;
                }
            }
        }
    }

    if closers.len() > 0 {
        return ParseResult::Incomplete(closers.iter().rev().collect());
    } else {
        return ParseResult::Ok;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseResult {
    Ok,
    Corrupted { expected: char, actual: char },
    Incomplete(String),
    Overflow,
}

pub fn score_corrupted(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

pub fn score_incomplete(s: &str) -> u128 {
    let mut score = 0;
    s.chars().for_each(|c| {
        score *= 5;
        score += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        };
    });
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ok() {
        let chunk = "[<>({}){}[([])<>]]";
        assert_eq!(ParseResult::Ok, parse_chunk(chunk));
    }

    #[test]
    fn parse_corrupted() {
        let chunk = "{([(<{}[<>[]}>{[]{[(<()>";
        let expected = ParseResult::Corrupted { expected: ']', actual: '}' };
        assert_eq!(expected, parse_chunk(chunk));
    }

    #[test]
    fn parse_incomplete() {
        let chunk = "{(<[]";
        let expected = ParseResult::Incomplete(String::from(">)}"));
        assert_eq!(expected, parse_chunk(chunk));
    }

    #[test]
    fn parse_overflow() {
        let chunk = ">";
        let expected = ParseResult::Overflow;
        assert_eq!(expected, parse_chunk(chunk));
    }

    #[test]
    fn score_incomplete_high() {
        let completion = "}}>}>))))";
        let expected = 1_480_781;
        assert_eq!(expected, score_incomplete(completion));
    }
}
