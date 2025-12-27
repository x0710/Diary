use std::fmt::Display;

#[derive(Debug)]
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