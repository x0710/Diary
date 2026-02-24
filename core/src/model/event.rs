use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Event {
    pub instruct: String,
}
impl Event {
    pub fn new(instruct: &str) -> Event {
        Event {
            instruct: instruct.to_owned(),
        }
    }
}
impl From<String> for Event {
    fn from(instruct: String) -> Event {
        Event { instruct }
    }
}
impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.instruct)
    }
}
impl Default for Event {
    fn default() -> Self {
        Self {
            instruct: "".to_string(),
        }
    }
}