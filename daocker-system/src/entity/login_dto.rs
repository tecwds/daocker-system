use lombok::{Getter, Setter};
use serde::{Deserialize, Serialize};

#[derive(Getter, Setter, Deserialize, Serialize)]
pub struct LoginDTO {
    email: Option<String>,
    student_id: Option<String>,
    password: String
}


