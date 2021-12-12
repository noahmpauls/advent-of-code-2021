use std::collections::{HashMap, HashSet};

pub struct Cave(pub String, pub String);

pub struct CaveSystem {
    adjacency: HashMap<String, Vec<String>>,
    big: HashSet<String>,
    small: HashSet<String>,
}

impl CaveSystem {
    pub fn new(caves: Vec<Cave>) -> CaveSystem {
        let mut adjacency = HashMap::new();
        let mut big = HashSet::new();
        let mut small = HashSet::new();

        for cave in caves.iter() {
            for c in [&cave.0, &cave.1] {
                if c.to_lowercase() == *c {
                    small.insert(c.clone());
                } else if c.to_uppercase() == *c {
                    big.insert(c.clone());
                } else {
                    panic!("invalid cave {}", c);
                }
            }

            for adj in [(&cave.0, &cave.1), (&cave.1, &cave.0)] {
                let entry = adjacency.entry(adj.0.clone()).or_insert(Vec::new());
                if !entry.contains(adj.1) {
                    entry.push(adj.1.clone());
                }
            }
        }

        CaveSystem { adjacency, big, small }
    }

    pub fn unique_paths(&self, start: &str, end: &str) -> Vec<Vec<String>> {
        let mut visited = HashSet::new();
        visited.insert(start);
        self.find_unique_paths(start, end, visited)
    }

    fn find_unique_paths(&self, start: &str, end: &str, visited: HashSet<&str>) -> Vec<Vec<String>> {
        let mut paths = Vec::new();

        if start != end {
            for node in self.adjacency.get(start).unwrap() {
                if !visited.contains(&node[..]) {
                    let mut new_visited = visited.clone();
                    if self.small.contains(&node[..]) {
                        new_visited.insert(node);
                    }
                    for path in self.find_unique_paths(&node[..], end, new_visited).iter_mut() {
                        let mut new_path = vec![String::from(start)];
                        new_path.append(path);
                        paths.push(new_path);
                    }
                }
            }
        } else {
            paths.push(vec![String::from(end)]);
        }

        paths
    }

    pub fn unique_paths_twice(&self, start: &str, end: &str) -> Vec<Vec<String>> {
        let mut visited = HashSet::new();
        visited.insert(start);
        self.find_unique_paths_twice(start, end, visited, false)
    }

    fn find_unique_paths_twice(&self, start: &str, end: &str, visited: HashSet<&str>, twice: bool) -> Vec<Vec<String>> {
        let mut paths = Vec::new();

        if start != end {
            for node in self.adjacency.get(start).unwrap() {
                if !visited.contains(&node[..]) {
                    let mut new_visited = visited.clone();
                    if self.small.contains(&node[..]) {
                        new_visited.insert(node);
                    }
                    for path in self.find_unique_paths_twice(&node[..], end, new_visited, twice).iter_mut() {
                        let mut new_path = vec![String::from(start)];
                        new_path.append(path);
                        paths.push(new_path);
                    }
                } else if !twice && node != "start" {
                    for path in self.find_unique_paths_twice(&node[..], end, visited.clone(), true).iter_mut() {
                        let mut new_path = vec![String::from(start)];
                        new_path.append(path);
                        paths.push(new_path);
                    }
                }
            }
        } else {
            paths.push(vec![String::from(end)]);
        }

        paths
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
