use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coord {
    pub r: i32,
    pub c: i32,
}

impl Coord {
    pub fn new(r: i32, c: i32) -> Coord {
        Coord { r, c }
    }

    pub fn read_adj(&self) -> impl Iterator<Item=Coord> {
        let r = self.r;
        let c = self.c;
        (-1..=1).flat_map(move |dr| {
            (-1..=1).map(move |dc| {
                Coord::new(r + dr, c + dc)
            })
        })
    }
}

pub struct ImageEnhancer {
    pixels: HashSet<Coord>,
    enhancer: [bool; 512],
    inverted: bool,
}

impl ImageEnhancer {
    pub fn new(image: &Vec<&str>, enhance_string: &str) -> Self {
        let pixels = image.iter().enumerate().flat_map(|(r, row)| {
            row.chars().enumerate().filter_map(move |(c, char)| {
                if char == '#' {
                    Some(Coord::new(r.try_into().unwrap(), c.try_into().unwrap()))
                } else {
                    None
                }
            })
        }).collect();

        assert_eq!(512, enhance_string.chars().count());
        let mut enhancer = [false; 512];
        for (i, char) in enhance_string.chars().enumerate() {
            enhancer[i] = match char {
                '#' => true,
                '.' => false,
                _ => panic!("invalid character in enhance_string"),
            };
        }

        Self { pixels, enhancer, inverted: false }
    }

    pub fn enhance(&mut self) {
        //  for each lit pixels:
        //      for each pixel around the lit pixel:
        //          construct the index from the block
        //          insert the new lit pixel
        //          memoize the result

        let mut lit = HashSet::new();
        let mut dark = HashSet::new();
        let mut memo = HashSet::new();

        for pixel in self.pixels.iter() {
            for adj in pixel.read_adj() {
                if !memo.contains(&adj) {
                    let index: String = adj.read_adj()
                    .map(|c| {
                        match self.is_lit(c) {
                            true => '1',
                            false => '0',
                        }
                    }).collect();
                    let index: usize = usize::from_str_radix(&index, 2).unwrap();
                    if self.enhancer[index] {
                        lit.insert(adj);
                    } else {
                        dark.insert(adj);
                    }
                    memo.insert(adj);
                }
            }
        }

        if self.inverted {
            if !self.enhancer[511] {
                self.pixels = lit;
                self.inverted = false;
            } else {
                self.pixels = dark;
            }
        } else {
            if self.enhancer[0] {
                self.pixels = dark;
                self.inverted = true;
            } else {
                self.pixels = lit;
            }
        }
    }

    fn is_lit(&self, c: Coord) -> bool {
        self.inverted ^ self.pixels.contains(&c)
    }

    pub fn lit_count(&self) -> Option<usize> {
        if self.inverted {
            None
        } else {
            Some(self.pixels.len())
        }
    }

    pub fn image(&self) -> String {
        let mut result = String::new();
        let (min, max) = self.bounds();

        for r in min.r..=max.r {
            for c in min.c..=max.c {
                if self.is_lit(Coord::new(r, c)) {
                    result.push('#');
                } else {
                    result.push('.');
                }
            }
            result.push('\n');
        }

        String::from(result.trim())
    }

    fn bounds(&self) -> (Coord, Coord) {
        if self.pixels.len() == 0 {
            (Coord::new(0, 0), Coord::new(0, 0))
        } else {
            let min = self.pixels.iter().cloned().reduce(|a, b| {
                Coord::new(std::cmp::min(a.r, b.r), std::cmp::min(a.c, b.c))
            }).unwrap();
            let max = self.pixels.iter().cloned().reduce(|a, b| {
                Coord::new(std::cmp::max(a.r, b.r), std::cmp::max(a.c, b.c))
            }).unwrap();

            (min, max)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_read_adj() {
        let c = Coord::new(0, 0);
        let expected = vec![
            (-1, -1), (-1, 0), (-1, 1),
            ( 0, -1), ( 0, 0), ( 0, 1),
            ( 1, -1), ( 1, 0), ( 1, 1),
        ].into_iter().map(|p| Coord::new(p.0, p.1));

        for (expected, actual) in expected.zip(c.read_adj()) {
            assert_eq!(expected, actual);
        }
    }
}
