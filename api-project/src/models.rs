use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct Question {
    // TODO: add a public `title` field of type String
    pub title: String,
    // TODO: add a public `description` field of type String
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuestionDetail {
    // TODO: add a public `question_uuid` field of type String
    pub question_uuid: String,
    // TODO: add a public `title` field of type String
    pub title: String,
    // TODO: add a public `description` field of type String
    pub description: String,
    // TODO: add a public `created_at` field of type String
    pub created_at: String,
}

// TODO: create a struct called `QuestionId`
//       derive the following traits: Serialize, Deserialize
//       add a public `question_uuid` field of type String
#[derive(Serialize, Deserialize, FromForm)]
pub struct QuestionId {
    pub question_uuid: String,
}

// ----------

// TODO: create a struct called `Answer`
//       derive the following traits: Serialize, Deserialize
//       add a public `question_uuid` field of type String
//       add a public `content` field of type String
#[derive(Serialize, Deserialize)]
pub struct Answer {
    pub question_uuid: String,
    pub content: String,
}

// TODO: create a struct called `AnswerDetail`
//       derive the following traits: Serialize, Deserialize, Debug, Clone, PartialEq
//       add a public `answer_uuid` field of type String
//       add a public `question_uuid` field of type String
//       add a public `content` field of type String
//       add a public `created_at` field of type String
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerDetail {
    pub answer_uuid: String,
    pub question_uuid: String,
    pub content: String,
    pub created_at: String,
}

// TODO: create a struct called `AnswerId`
//       derive the following traits: Serialize, Deserialize
//       add a public `answer_uuid` field of type String

#[derive(Serialize, Deserialize)]
pub struct AnswerId {
    pub answer_uuid: String,
}

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Invalid UUID provided: {0}")]
    InvalidUUID(String),
    #[error("Database error occurred")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

// source: https://www.postgresql.org/docs/current/errcodes-appendix.html
pub mod postgres_error_codes {
    pub const FOREIGN_KEY_VIOLATION: &str = "23503";
}
