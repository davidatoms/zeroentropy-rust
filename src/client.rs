use crate::error::{Error, Result};
use reqwest::{Client as HttpClient, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "https://api.zeroentropy.dev/v1";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);
const DEFAULT_MAX_RETRIES: u32 = 2;

/// ZeroEntropy API client
#[derive(Clone)]
pub struct Client {
    http_client: HttpClient,
    api_key: String,
    base_url: String,
    max_retries: u32,
}

impl Client {
    /// Create a new ZeroEntropy client
    ///
    /// # Arguments
    /// * `api_key` - Your ZeroEntropy API key (can also use ZEROENTROPY_API_KEY env var)
    ///
    /// # Example
    /// ```no_run
    /// use zeroentropy::Client;
    ///
    /// let client = Client::new("your-api-key").unwrap();
    /// ```
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::builder().api_key(api_key).build()
    }

    /// Create a new client from environment variable
    ///
    /// Reads the API key from the ZEROENTROPY_API_KEY environment variable
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("ZEROENTROPY_API_KEY")
            .map_err(|_| Error::InvalidApiKey)?;
        Self::new(api_key)
    }

    /// Create a client builder for advanced configuration
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Make a POST request to the API
    pub(crate) async fn post<T, R>(&self, endpoint: &str, body: &T) -> Result<R>
    where
        T: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        
        let mut attempts = 0;
        loop {
            let response = self
                .http_client
                .post(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(body)
                .send()
                .await?;

            let status = response.status();
            
            // Check if we should retry
            if attempts < self.max_retries && Self::should_retry(status.as_u16()) {
                attempts += 1;
                let delay = Self::calculate_retry_delay(attempts);
                tokio::time::sleep(delay).await;
                continue;
            }

            return Self::handle_response(response).await;
        }
    }

    /// Handle the API response
    async fn handle_response<R: DeserializeOwned>(response: Response) -> Result<R> {
        let status = response.status();
        
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let status_code = status.as_u16();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            
            // Try to parse error message from JSON response
            let message = serde_json::from_str::<serde_json::Value>(&error_text)
                .ok()
                .and_then(|v| v.get("message").and_then(|m| m.as_str()).map(String::from))
                .unwrap_or(error_text);
            
            Err(Error::from_status(status_code, message))
        }
    }

    /// Check if a status code should trigger a retry
    fn should_retry(status: u16) -> bool {
        matches!(status, 408 | 409 | 429) || status >= 500
    }

    /// Calculate exponential backoff delay
    fn calculate_retry_delay(attempt: u32) -> Duration {
        let base_delay = 500; // milliseconds
        let max_delay = 8000; // milliseconds
        let delay = base_delay * 2_u64.pow(attempt - 1);
        Duration::from_millis(delay.min(max_delay))
    }
}

/// Builder for constructing a ZeroEntropy client with custom options
#[derive(Default)]
pub struct ClientBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
}

impl ClientBuilder {
    /// Set the API key
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Set a custom base URL (useful for testing)
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Set the request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set the maximum number of retries
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    /// Build the client
    pub fn build(self) -> Result<Client> {
        let api_key = self.api_key
            .or_else(|| std::env::var("ZEROENTROPY_API_KEY").ok())
            .ok_or(Error::InvalidApiKey)?;

        let base_url = self.base_url
            .or_else(|| std::env::var("ZEROENTROPY_BASE_URL").ok())
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());

        let timeout = self.timeout.unwrap_or(DEFAULT_TIMEOUT);
        let max_retries = self.max_retries.unwrap_or(DEFAULT_MAX_RETRIES);

        let http_client = HttpClient::builder()
            .timeout(timeout)
            .build()?;

        Ok(Client {
            http_client,
            api_key,
            base_url,
            max_retries,
        })
    }
}
