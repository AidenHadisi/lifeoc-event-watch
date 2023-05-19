use chat_gpt_rs::request::Response;
use log::info;
use serde::{Deserialize, Serialize};

use crate::result::{Error, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub title: String,
    pub description: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventList(pub Vec<Event>);

impl TryFrom<Response> for EventList {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self> {
        let message = value
            .choices
            .get(0)
            .ok_or(Error::Parser("No choices found".to_string()))?
            .message
            .content
            .as_str();

        info!("Response: {:?}", message);

        let result = serde_json::from_str::<Vec<Event>>(message)
            .map_err(|e| Error::Parser(e.to_string()))?;

        Ok(Self(result))
    }
}

impl std::ops::Deref for EventList {
    type Target = Vec<Event>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
