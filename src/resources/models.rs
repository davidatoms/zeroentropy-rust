use crate::client::Client;
use crate::error::Result;
use crate::types::{RerankDocument, RerankResponse};
use serde::Serialize;

/// Models resource for reranking operations
pub struct Models<'a> {
    client: &'a Client,
}

impl<'a> Models<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Rerank documents based on relevance to a query
    ///
    /// # Arguments
    /// * `query` - The query to rank documents against
    /// * `documents` - List of documents to rerank
    /// * `model_id` - Optional model ID (defaults to best available model)
    /// * `top_k` - Optional number of top results to return
    ///
    /// # Example
    /// ```no_run
    /// # use zeroentropy_community::{Client, RerankDocument};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::from_env()?;
    /// let documents = vec![
    ///     RerankDocument {
    ///         id: "doc1".to_string(),
    ///         text: "Rust is a systems programming language".to_string(),
    ///     },
    ///     RerankDocument {
    ///         id: "doc2".to_string(),
    ///         text: "Python is a high-level programming language".to_string(),
    ///     },
    /// ];
    /// 
    /// let response = client.models().rerank(
    ///     "systems programming",
    ///     documents,
    ///     None,
    ///     None,
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn rerank(
        &self,
        query: impl Into<String>,
        documents: Vec<RerankDocument>,
        model_id: Option<String>,
        top_k: Option<u32>,
    ) -> Result<RerankResponse> {
        #[derive(Serialize)]
        struct Request {
            query: String,
            documents: Vec<RerankDocument>,
            #[serde(skip_serializing_if = "Option::is_none")]
            model_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            top_k: Option<u32>,
        }

        let body = Request {
            query: query.into(),
            documents,
            model_id,
            top_k,
        };

        self.client.post("/models/rerank", &body).await
    }
}
