use lombok::{Getter, Setter};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq)]
pub enum Gender {
    /// 男性
    Male,

    /// 女性，
    Female,

    /// 未知
    UnKnown,
}

impl From<u32> for Gender {
    fn from(value: u32) -> Self {
        match value {
            10 => Self::Male,
            11 => Self::Female,
            _ => Self::UnKnown,
        }
    }
}

impl From<&'static str> for Gender {
    fn from(value: &'static str) -> Self {
        match value {
            "男" => Self::Male,
            "女" => Self::Female,
            _ => Self::UnKnown
        }
    }
}

impl Gender {
    #[allow(dead_code)]
    pub fn validate(val: u32) -> bool {
        Self::UnKnown != Self::from(val)
    }

    #[allow(dead_code)]
    pub fn get_u32(&self) -> u32 {
        match self {
            Gender::Male => 10,
            Gender::Female => 11,
            Gender::UnKnown => 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Getter, Setter, sqlx::FromRow)]
pub struct Student {
    /// ID 主键
    pub(crate) id: u64,

    /// 学号
    pub(crate) student_id: String,
    
    /// 邮箱
    pub(crate) email: String,

    /// 密码
    pub(crate) password: String,

    /// 姓
    pub(crate) full_name: String,

    /// 年龄
    pub(crate) age: u32,

    /// 性别
    pub(crate) gender: u32,

    /// 学院
    pub(crate) college: String,

    /// 专业
    pub(crate) major: String,

    /// 年级
    pub(crate) grade: u32,

    /// 班级
    pub(crate) class: u32,
}
