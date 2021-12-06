pub struct Lanternfish {
    timer: u8,
}

impl Lanternfish {
    pub fn new() -> Lanternfish {
        Lanternfish { timer: 8 }
    }

    pub fn from(timer: u8) -> Lanternfish {
        Lanternfish { timer }
    }

    pub fn timer(&self) -> u8 {
        self.timer
    }

    pub fn step(&mut self) -> Option<Lanternfish> {
        if self.timer == 0 {
            self.timer = 6;
            Some(Lanternfish::new())
        } else {
            self.timer -= 1;
            None
        }
    }
}


pub struct LanternfishSchool {
    counts: [u128; 9],
    z: usize,
}

impl LanternfishSchool {
    pub fn from(timers: &[u8]) -> LanternfishSchool {
        let mut counts = [0; 9];
        for timer in timers {
            assert!(*timer <= 8);
            counts[*timer as usize] += 1;
        }

        LanternfishSchool { counts, z: 0 }
    }

    pub fn step(&mut self) {
        let repro_count = self.counts[self.z];
        self.counts[self.z] = 0;

        // increment before reproducing, since new fish will have timers
        // relative to next zero index
        self.z = Self::wrap(self.z + 1);
        self.counts[Self::wrap(self.z + 6)] += repro_count;
        self.counts[Self::wrap(self.z + 8)] += repro_count;
    }

    fn wrap(index: usize) -> usize {
        index % 9
    }

    pub fn size(&self) -> u128 {
        self.counts.iter().sum()
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
