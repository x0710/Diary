#[derive(Debug)]
pub struct Event {
    pub instruct: String,
}
impl Event {
    pub fn new(instruct: String) -> Event {
        Event { instruct }
    }
}