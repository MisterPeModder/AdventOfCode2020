use regex::Regex;
pub struct Password {
    n1: u32,
    n2: u32,
    character: char,
    password: String,
}

impl Password {
    pub fn new(n1: u32, n2: u32, character: char, password: &str) -> Self {
        Password {
            n1, n2, character,
            password: password.to_owned(),
        }
    }

    pub fn is_valid1(&self) -> bool {
        self.n1 <= self.n2 && (self.n1..=self.n2).contains(&(
            self.password.chars().filter(|&c| c == self.character).count() as u32
        ))
    }

    pub fn is_valid2(&self) -> bool {
        self.password.chars().enumerate().filter(|&(i, c)| {
            c == self.character && (i + 1 == self.n1 as usize || i + 1 == self.n2 as usize)
        }).count() == 1
    }
}

#[aoc_generator(day02)]
pub fn day02_gen(input: &str) -> Vec<Password> {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();
    }
    input.lines().filter_map(|l| {
        let caps = PATTERN.captures(l)?;

        Some(Password::new(
            caps[1].parse().ok()?,
            caps[2].parse().ok()?,
            caps[3].chars().nth(0).unwrap(),
            &caps[4]
        ))
    }).collect()
}

#[aoc(day02, part1)]
pub fn  day02_part1(input: &[Password]) -> u32 {
    input.iter().filter(|&pwd| pwd.is_valid1()).count() as u32
}

#[aoc(day02, part2)]
pub fn  day02_part2(input: &[Password]) -> u32 {
    input.iter().filter(|&pwd| pwd.is_valid2()).count() as u32
}

#[cfg(test)]
mod tests {
    use super::Password;

    #[test]
    fn test_part1() {
        let p1 = Password::new(1, 3, 'a', "adcde");
        let p2 = Password::new(1, 3, 'b', "cdefg");
        let p3 = Password::new(2, 9, 'c', "ccccccccc");
        
        assert!(p1.is_valid1());
        assert!(!p2.is_valid1());
        assert!(p3.is_valid1());
    }

    #[test]
    fn test_part2() {
        let p1 = Password::new(1, 3, 'a', "adcde");
        let p2 = Password::new(1, 3, 'b', "cdefg");
        let p3 = Password::new(2, 9, 'c', "ccccccccc");
        
        assert!(p1.is_valid2());
        assert!(!p2.is_valid2());
        assert!(!p3.is_valid2());
    }
}