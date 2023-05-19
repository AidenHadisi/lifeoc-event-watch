use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;

use crate::event::Event;
use crate::result::{Error::*, Result};

/// An API that can create events.
pub trait EventsAPI {
    /// Creates events.
    async fn create_event(&self, event: &Event) -> Result<()>;
}

/// An API that can create events in WordPress.
pub struct WPEvents {
    client: Client,
}

impl WPEvents {
    /// Creates a new WPEvents.
    pub fn new(username: &str, password: &str) -> Self {
        let credentials = base64::encode(&format!("{}:{}", username, password));

        // Create the authorization header value
        let auth_header_value = HeaderValue::from_str(&format!("Basic {}", credentials)).unwrap();

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, auth_header_value);

        let client = Client::builder().default_headers(headers).build().unwrap();

        Self { client }
    }
}

impl EventsAPI for WPEvents {
    async fn create_event(&self, event: &Event) -> Result<()> {
        self.client
            .post("https://lifeoc.org/wp-json/tribe/events/v1/events")
            .json(&event)
            .send()
            .await
            .map_err(|e| ApiError(e.to_string()))?
            .status()
            .is_success()
            .then(|| Ok(()))
            .unwrap_or_else(|| Err(ApiError("Failed to create event".to_string())))
    }
}
