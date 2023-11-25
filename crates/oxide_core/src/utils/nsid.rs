use std::fmt::{self, Display, Formatter};
use std::ops;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub struct NSID {
    parts: Vec<String>
}

impl NSID {
    const SEPARATOR: &'static str = ":";

    pub fn from_parts(parts: Vec<String>) -> Self {
        Self {
            parts,
        }
    }
}

impl ops::Add<String> for NSID {
    type Output = Self;

    fn add(self, rhs: String) -> Self::Output {
        let mut new_parts = self.parts.clone();
        new_parts.push(rhs);
        Self::from_parts(new_parts)
    }
}

impl ops::Add<&str> for NSID {
    type Output = Self;

    fn add(self, rhs: &str) -> Self::Output {
        let mut new_parts = self.parts.clone();
        new_parts.push(String::from(rhs))
        ;
        Self::from_parts(new_parts)
    }
}

impl ops::Add<NSID> for NSID {
    type Output = Self;

    fn add(self, rhs: NSID) -> Self::Output {
        let mut new_parts = self.parts.clone();
        new_parts.append(&mut rhs.parts.clone());
        Self::from_parts(new_parts)
    }
}

impl FromStr for NSID {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<String> = s.split(NSID::SEPARATOR).map(String::from).collect();
        Ok(Self {
            parts,
        })
    }
}

impl Display for NSID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.parts.join(NSID::SEPARATOR))
    }
}