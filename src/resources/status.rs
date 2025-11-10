use crate::client::Client;
use crate::error::Result;
use crate::types::StatusResponse;

/// Status resource for checking account status
pub struct Status<'a> {
    client: &'a Client,
}

impl<'a> Status<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get account status including document and collection counts
    ///
    /// # Example
    /// ```no_run
    /// # use zeroentropy_community::Client;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::from_env()?;
    /// let status = client.status().get_status().await?;
    /// println!("Documents: {}, Collections: {}", 
    ///     status.num_documents, 
    ///     status.num_collections
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_status(&self) -> Result<StatusResponse> {
        self.client.post("/status/get-status", &serde_json::json!({})).await
    }
}
