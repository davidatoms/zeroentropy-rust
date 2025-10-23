use crate::client::Client;
use crate::error::Result;
use crate::types::{CollectionListResponse, CollectionResponse};
use serde::Serialize;

/// Collections resource for managing document collections
pub struct Collections<'a> {
    client: &'a Client,
}

impl<'a> Collections<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Add a new collection
    ///
    /// # Arguments
    /// * `collection_name` - Name of the collection to create
    ///
    /// # Example
    /// ```no_run
    /// # use zeroentropy::Client;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::from_env()?;
    /// client.collections().add("my_collection").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add(&self, collection_name: impl Into<String>) -> Result<CollectionResponse> {
        #[derive(Serialize)]
        struct Request {
            collection_name: String,
        }

        let body = Request {
            collection_name: collection_name.into(),
        };

        self.client.post("/collections/add-collection", &body).await
    }

    /// Delete a collection
    ///
    /// # Arguments
    /// * `collection_name` - Name of the collection to delete
    ///
    /// # Example
    /// ```no_run
    /// # use zeroentropy::Client;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::from_env()?;
    /// client.collections().delete("my_collection").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, collection_name: impl Into<String>) -> Result<CollectionResponse> {
        #[derive(Serialize)]
        struct Request {
            collection_name: String,
        }

        let body = Request {
            collection_name: collection_name.into(),
        };

        self.client.post("/collections/delete-collection", &body).await
    }

    /// Get list of all collections
    ///
    /// # Example
    /// ```no_run
    /// # use zeroentropy::Client;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::from_env()?;
    /// let response = client.collections().get_list().await?;
    /// for collection in response.collections {
    ///     println!("Collection: {}", collection);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_list(&self) -> Result<CollectionListResponse> {
        self.client.post("/collections/get-collection-list", &serde_json::json!({})).await
    }
}
