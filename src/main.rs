#![feature(async_fn_in_trait)]
#![allow(deprecated)]
#![allow(incomplete_features)]
use futures::future::try_join_all;
use lambda_http::{run, service_fn, Body, Request, Response};
use log::info;
use parser::ScheduleParser;
use result::{Error, Result};
use std::env;

use crate::events_api::EventsAPI;

mod event;
mod events_api;
mod parser;
mod prompt;
mod result;

async fn function_handler(event: Request) -> Result<Response<Body>> {
    let parser =
        parser::GPTScheduleParser::new(env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not set"));

    let api = events_api::WPEvents::new(
        &env::var("WP_USERNAME").expect("WP_USERNAME not set"),
        &env::var("WP_PASSWORD").expect("WP_PASSWORD not set"),
    );

    let Body::Text(text) = event.body() else {
        return Err(Error::InternalServer("Invalid body".to_string()));
    };

    info!("Received text: {}", text);

    let events = parser.parse(text).await?;

    info!("Parsed events: {:?}", events);

    let tasks = events.iter().map(|event| api.create_event(event));

    //await all return error if any of the tasks return error
    try_join_all(tasks).await?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("Recieved".into())
        .map_err(|e| Error::InternalServer(e.to_string()))?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<()> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // this needs to be set to false, otherwise ANSI color codes will
        // show up in a confusing manner in CloudWatch logs.
        .with_ansi(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler))
        .await
        .map_err(|e| Error::InternalServer(e.to_string()))?;

    Ok(())
}
