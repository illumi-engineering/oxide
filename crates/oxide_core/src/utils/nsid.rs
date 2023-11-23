use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

trait TestTrait {
}

pub struct NSID {
    parts: Vec<String>
}

impl NSID {
    const SEPARATOR: &'static str = ":";
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