use std::fmt;
use std::error;

#[derive(Debug)]
pub struct EmptyError(pub String);

impl fmt::Display for EmptyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EmptyError error {}", self.0)
    }
}

impl error::Error for EmptyError {
    fn description(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictLog {
    pub from_lang: Option<String>,
    pub from: String,
    pub to_lang: Option<String>,
    pub to: String,
}

#[derive(Serialize, Deserialize)]
pub struct DictLogEntity {
    pub id: i64,
    pub from_lang: Option<String>,
    pub from: String,
    pub to_lang: Option<String>,
    pub to: String,
}
