#[macro_use]
extern crate rocket;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

// TODO: import log, pretty_env_logger, dotenv, and PgPoolOptions

mod cors;
mod handlers;
mod models;
mod persistence;
mod utils;

use cors::*;
use handlers::*;
use persistence::{
    answers_dao::{self, AnswersDao, AnswersDaoImpl},
    questions_dao::{self, QuestionsDao, QuestionsDaoImpl},
};
use sqlx::postgres::PgPoolOptions;

use anyhow::Context;

#[launch]
async fn rocket() -> _ {
    // TODO: Initialize pretty_env_logger
    pretty_env_logger::init();
    // TODO: Initialize dotenv
    // Load environment variables from .env file.
    // Fails if .env file not found, not readable or invalid.
    dotenvy::dotenv().expect("could not read environment from .env");
    //dotenvy::dotenv().ok()

    // Create a new PgPoolOptions instance with a maximum of 5 connections.
    // Use dotenv to get the database url.
    // Use the `unwrap` or `expect` method instead of handling errors. If an
    // error occurs at this stage the server should be terminated.
    // See examples on GitHub page: https://github.com/launchbadge/sqlx
    //let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            &dotenvy::var("DATABASE_URL") // std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
        )
        .await
        // .context("failed to connect to DATABASE_URL")?;
        .expect("failed to connect to DATABASE_URL");

    // Using slqx, execute a SQL query that selects all questions from the questions table.
    // Use the `unwrap` or `expect` method to handle errors. This is just some test code to
    // make sure we can connect to the database.
    // let recs = todo!();

    // example from https://github.com/launchbadge/sqlx/blob/main/examples/postgres/axum-social-with-tests/src/http/user.rs
    // sqlx::query!(
    //     // language=PostgreSQL
    //     r#"
    //         insert into "user"(username, password_hash)
    //         values ($1, $2)
    //     "#,
    //     username,
    //     password_hash
    // )
    // .execute(&*db)
    // .await
    // .map_err(|e| match e {
    //     sqlx::Error::Database(dbe) if dbe.constraint() == Some("user_username_key") => {
    //         Error::Conflict("username taken".into())
    //     }
    //     _ => e.into(),
    // })?;

    // let recs = sqlx::query!(
    //     r#"
    //         select question_uuid, title, description, created_at
    //         from questions
    //     "#
    // )
    // .fetch_all(&pool)
    // .await
    // .expect("could not retrieve all questions from the db");

    // info!("********* Question Records *********");
    // // TODO: Log recs with debug formatting using the info! macro
    // recs.iter().for_each(|question| info!("{:?}", question));

    let questions_dao = QuestionsDaoImpl::new(pool.clone());
    let answers_dao = AnswersDaoImpl::new(pool.clone());

    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                create_answer,
                read_answers,
                delete_answer
            ],
        )
        .attach(CORS)
        // The manage method allows us to add state to the state managed by this instance of Rocket. Then we can use this state in the handlers.
        .manage(Box::new(questions_dao) as Box<dyn QuestionsDao + Send + Sync>) // pass in `questions_dao` as a boxed trait object. hint: you must cast `questions_dao` to a trait object.
        .manage(Box::new(answers_dao) as Box<dyn AnswersDao + Send + Sync>) // pass in `answers_dao` as a boxed trait object. hint: you must cast `answers_dao` to a trait object.
}
