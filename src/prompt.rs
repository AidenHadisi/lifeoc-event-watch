use chrono::Local;
use std::fmt::Display;

/// A prompt to send to GPT-3.
pub struct Prompt(String);

impl Prompt {
    /// Creates a new prompt.
    pub fn new(text: &str) -> Self {
        let formatted_date = Local::now().format("%a %b %d %Y");

        let prompt = format!(
            "Following is an email that may contain my Church's weekly schedule. Schedule is explained in plain English. 
            Your job is to analyze this email and respond back with the weekly schedule in JSON format.
            If the date is not included, assume it is for current week given that today is {}. Time zone is PST. 
            If the email does not contain schedule, reply \"No Schedule Found\". 
            If email contains schedule only respond in json array format below. Use following template:
            [
            {{
                \"title\": \"[Short title of the event]\",
                \"description\": \"[2 or 3 sentence description of the event]\",
                \"start_date\":\"[ISO 8601 timestamp]\",
                \"end_date\":\"[start date + 2 hours ISO 8601 timestamp]\"
            }}
            ]

            ------------------

            {}
            ]",
            formatted_date, text);

        Self(prompt)
    }
}

impl Display for Prompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
