use rocket::State;
use rocket::{serde::json::Json, serde::uuid::Uuid};
use std::time::SystemTime;
use time::format_description;
use time::OffsetDateTime;

use crate::models::*;
use crate::persistence::answers_dao::AnswersDao;
use crate::persistence::questions_dao::QuestionsDao;
use crate::utils::now_as_string;

use self::handlers_inner::HandlerError;

mod handlers_inner;

#[derive(Responder)]
pub enum APIError {
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 500)]
    InternalError(String),
}

impl From<HandlerError> for APIError {
    fn from(value: HandlerError) -> Self {
        match value {
            HandlerError::BadRequest(s) => Self::BadRequest(s),
            HandlerError::InternalError(s) => Self::InternalError(s),
        }
    }
}

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
    // Example of how to add state to a route
    // TODO: fix compile time error importing QuestionsDao
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Result<Json<QuestionDetail>, APIError> {
    // TODO: update return type to be of type `Result`. Use `APIError` for the error case.
    // TODO: Replace the fake data below with a call to `handlers_inner::create_question`.
    // Return the result wrapped in JSON in the success case and an `APIError` in the error case.
    // NOTE: You can easily turn `HandlerError` into an `APIError` because of the From trait implementation above.
    // Json(QuestionDetail {
    //     question_uuid: Uuid::new_v4().to_string(),
    //     title: question.title.clone(),
    //     description: question.description.clone(),
    //     created_at: now_as_string(),
    // })
    let question_result = handlers_inner::create_question(question.0, questions_dao)
        .await
        .map_err(|err| APIError::from(err))?;
    Ok(Json(question_result))
}

#[get("/questions")]
pub async fn read_questions(
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>, // add the appropriate type annotation
) -> Result<Json<Vec<QuestionDetail>>, APIError> {
    // TODO: update return type to be of type `Result`. Use `APIError` for the error case.
    // TODO: Replace the fake data below with a call to `handlers_inner::read_questions`.
    // Return the result wrapped in JSON in the success case and an `APIError` in the error case.
    // NOTE: You can easily turn `HandlerError` into an `APIError` because of the From trait implementation above.
    let read_result = handlers_inner::read_questions(questions_dao)
        .await
        .map_err(|err| APIError::from(err))?;

    Ok(Json(read_result))
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(
    question_uuid: Json<QuestionId>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>, // add the appropriate type annotation
) -> Result<(), APIError> {
    // TODO: return `Result` from this function. Use a unit type in the success case and `APIError` in the error case.
    // TODO: Make a call to `handlers_inner::delete_question`.
    // Return a unit type in the success case and an `APIError` in the error case.
    // NOTE: You can easily turn `HandlerError` into an `APIError` because of the From trait implementation above.
    handlers_inner::delete_question(question_uuid.0, questions_dao)
        .await
        .map_err(|err| APIError::from(err))
}

// ---- CRUD for Answers ----

// TODO: Create a POST route to /answer which accepts an `Answer` and returns `AnswerDetail` as JSON.
//       The handler function should be called `create_answer`.
//
//       hint: this function should look very similar to the create_question function above
#[post("/answer", data = "<answer>")]
pub async fn create_answer(
    answer: Json<Answer>,
    // Example of how to add state to a route
    // TODO: fix compile time error importing AnswersDao
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>,
) -> Result<Json<AnswerDetail>, APIError> {
    // TODO: update return type to be of type `Result`. Use `APIError` for the error case.
    // TODO: Replace the fake data below with a call to `handlers_inner::create_answer`.
    // Return the result wrapped in JSON in the success case and an `APIError` in the error case.
    // NOTE: You can easily turn `HandlerError` into an `APIError` because of the From trait implementation above.
    let answer_response = handlers_inner::create_answer(answer.0, answers_dao)
        .await
        .map_err(|err| APIError::from(err))?;

    Ok(Json(answer_response))
}

// TODO: Create a GET route to /answers which accepts an `QuestionId` and returns a vector of `AnswerDetail` as JSON.
//       The handler function should be called `read_answers`.
//
//       hint: this function should look very similar to the read_questions function above
//#[get("/answers?<question_id>")]
#[get("/answers", data = "<question_uuid>")]
pub async fn read_answers(
    question_uuid: Json<QuestionId>,
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>, // add the appropriate type annotation
) -> Result<Json<Vec<AnswerDetail>>, APIError> {
    // TODO: update return type to be of type `Result`. Use `APIError` for the error case.
    // TODO: Replace the fake data below with a call to `handlers_inner::read_answers`.
    // Return the result wrapped in JSON in the success case and an `APIError` in the error case.
    // NOTE: You can easily turn `HandlerError` into an `APIError` because of the From trait implementation above.
    let read_response = handlers_inner::read_answers(question_uuid.0, answers_dao)
        .await
        .map_err(|err| APIError::from(err))?;
    Ok(Json(read_response))
}

// TODO: Create a DELETE route to /answer which accepts an `AnswerId` and does not return anything.
//       The handler function should be called `delete_answer`.
//
//       hint: this function should look very similar to the delete_question function above
#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(
    answer_uuid: Json<AnswerId>,
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>, // add the appropriate type annotation
) -> Result<(), APIError> {
    // TODO: return `Result` from this function. Use a unit type in the success case and `APIError` in the error case.
    // TODO: Make a call to `handlers_inner::delete_answer`.
    // Return a unit type in the success case and an `APIError` in the error case.
    // NOTE: You can easily turn `HandlerError` into an `APIError` because of the From trait implementation above.
    handlers_inner::delete_answer(answer_uuid.0, answers_dao)
        .await
        .map_err(|err| APIError::from(err))
}
