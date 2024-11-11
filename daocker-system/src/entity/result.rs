use lombok::{Getter, Setter};
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Getter, Setter)]
pub struct ResultEntity<T> {
    code: u32,
    message: String,
    data: Option<T>,
}

impl<T> ResultEntity<T> {
    /// .
    pub fn success(data: Option<T>) -> Self {
        Self {
            code: 200,
            message: String::from("ok"),
            data,
        }
    }

    pub fn error(msg: &str) -> Self {
        Self {
            code: 400,
            message: String::from(msg),
            data: None,
        }
    }
}
