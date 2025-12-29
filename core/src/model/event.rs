use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Event {
    pub instruct: String,
}
impl Event {
    pub fn new(instruct: &str) -> Event {
        let s = instruct.to_string();
        Event {
            instruct: s,
        }
    }
}
impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.instruct)
    }
}
impl FromStr for Event {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Event::new(s))
    }
}
impl Default for Event {
    fn default() -> Self {
        Self {
            instruct: "".to_string(),
        }
    }
}