use std::fmt;
use std::error;
use serde::ser;

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

#[derive(Serialize, Default)]
pub struct RErrorItem<E: ser::Serialize> {
    pub code: Option<String>,
    pub source: Option<String>,
    pub title: String,
    pub detail: Option<String>,
    pub data: Option<E>,
}

impl<E: ser::Serialize + Default> RErrorItem<E> {
    pub fn builder() -> ErrorItemBuilder<E> {
        ErrorItemBuilder::default()
    }
}

#[derive(Serialize, Default)]
pub struct ErrorItemBuilder<E: ser::Serialize> {
    code: Option<String>,
    source: Option<String>,
    title: String,
    detail: Option<String>,
    data: Option<E>,
}

impl<E: ser::Serialize> ErrorItemBuilder<E> {
    pub fn code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn data(mut self, data: E) -> Self {
        self.data = Some(data);
        self
    }

    pub fn detail(mut self, detail: String) -> Self {
        self.detail = Some(detail);
        self
    }

    pub fn build(self) -> RErrorItem<E> {
        RErrorItem {
            code: self.code,
            title: self.title,
            source: self.source,
            detail: self.detail,
            data: self.data,
        }
    }
}

#[derive(Serialize)]
pub struct RError<E: ser::Serialize> {
    pub errors: Vec<RErrorItem<E>>,
}

#[derive(Serialize)]
pub struct ROk<T: ser::Serialize> {
    pub data: T,
}
