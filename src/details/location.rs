use std::fmt;

#[derive(Clone, Eq, PartialEq)]
pub struct Location {
    line: usize,
    char: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.char)
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Location {
    pub fn new(line: usize, char: usize) -> Self {
        Self { line, char }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Region {
    start: Location,
    end: Location,
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

impl fmt::Debug for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Region {
    pub fn new(start: Location, end: Location) -> Self {
        Self { start, end }
    }
}
