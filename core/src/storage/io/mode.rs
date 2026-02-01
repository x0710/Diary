use std::str::FromStr;

pub enum Format {
    JSON,
    CSV,
}
impl FromStr for Format {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "json" => Ok(Format::JSON),
            "csv" => Ok(Format::CSV),
            _ => Err(()),
        }
    }
}