use crate::result::{Error::*, Result};
use crate::{event::EventList, prompt::Prompt};
use chat_gpt_rs::{
    prelude::Api,
    request::{Message, Model, Request},
    token::Token,
};

/// A tool that can parse a text and create a schedule from it.
pub trait ScheduleParser {
    /// Parses the text and returns a schedule.
    async fn parse(&self, text: &str) -> Result<EventList>;
}

/// Schedule parser that uses GPT-3 to parse the text.
pub struct GPTScheduleParser {
    client: Api,
}

impl GPTScheduleParser {
    /// Creates a new GPTScheduleParser.
    pub fn new(api_key: impl ToString) -> Self {
        Self {
            client: Api::new(Token::new(api_key)),
        }
    }
}

impl ScheduleParser for GPTScheduleParser {
    async fn parse(&self, text: &str) -> Result<EventList> {
        let prompt = Prompt::new(text);

        self.client
            .chat(Request {
                model: Model::Gpt35Turbo,
                messages: vec![Message {
                    content: prompt.to_string(),
                    role: "user".to_string(),
                }],
                ..Default::default()
            })
            .await
            .map_err(|e| ParserError(e.to_string()))?
            .try_into()
    }
}
