use crate::client::Client;
use crate::error::Result;
use crate::types::{
    Filter, LatencyMode, TopDocumentsResponse, TopPagesResponse, TopSnippetsResponse,
};
use serde::Serialize;

/// Queries resource for searching documents
pub struct Queries<'a> {
    client: &'a Client,
}

impl<'a> Queries<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Search for top documents matching a query
    ///
    /// # Arguments
    /// * `collection_name` - Name of the collection to search
    /// * `query` - Natural language query
    /// * `k` - Number of documents to return (1-2048)
    /// * `filter` - Optional metadata filter
    /// * `include_metadata` - Whether to include metadata in results
    /// * `latency_mode` - Latency/quality tradeoff
    /// * `reranker` - Optional reranker model ID
    pub async fn top_documents(
        &self,
        collection_name: impl Into<String>,
        query: impl Into<String>,
        k: u32,
        filter: Option<Filter>,
        include_metadata: Option<bool>,
        latency_mode: Option<LatencyMode>,
        reranker: Option<String>,
    ) -> Result<TopDocumentsResponse> {
        #[derive(Serialize)]
        struct Request {
            collection_name: String,
            query: String,
            k: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            filter: Option<Filter>,
            #[serde(skip_serializing_if = "Option::is_none")]
            include_metadata: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            latency_mode: Option<LatencyMode>,
            #[serde(skip_serializing_if = "Option::is_none")]
            reranker: Option<String>,
        }

        let body = Request {
            collection_name: collection_name.into(),
            query: query.into(),
            k,
            filter,
            include_metadata,
            latency_mode,
            reranker,
        };

        self.client.post("/queries/top-documents", &body).await
    }

    /// Search for top pages matching a query
    ///
    /// # Arguments
    /// * `collection_name` - Name of the collection to search
    /// * `query` - Natural language query
    /// * `k` - Number of pages to return (1-1024)
    /// * `filter` - Optional metadata filter
    /// * `include_content` - Whether to include page content
    /// * `latency_mode` - Latency/quality tradeoff
    pub async fn top_pages(
        &self,
        collection_name: impl Into<String>,
        query: impl Into<String>,
        k: u32,
        filter: Option<Filter>,
        include_content: Option<bool>,
        latency_mode: Option<LatencyMode>,
    ) -> Result<TopPagesResponse> {
        #[derive(Serialize)]
        struct Request {
            collection_name: String,
            query: String,
            k: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            filter: Option<Filter>,
            #[serde(skip_serializing_if = "Option::is_none")]
            include_content: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            latency_mode: Option<LatencyMode>,
        }

        let body = Request {
            collection_name: collection_name.into(),
            query: query.into(),
            k,
            filter,
            include_content,
            latency_mode,
        };

        self.client.post("/queries/top-pages", &body).await
    }

    /// Search for top snippets matching a query
    ///
    /// # Arguments
    /// * `collection_name` - Name of the collection to search
    /// * `query` - Natural language query
    /// * `k` - Number of snippets to return
    /// * `filter` - Optional metadata filter
    /// * `include_document_metadata` - Whether to include document metadata
    /// * `precise_responses` - Longer snippets (around 2000 chars vs 200 chars)
    /// * `reranker` - Optional reranker model ID
    pub async fn top_snippets(
        &self,
        collection_name: impl Into<String>,
        query: impl Into<String>,
        k: u32,
        filter: Option<Filter>,
        include_document_metadata: Option<bool>,
        precise_responses: Option<bool>,
        reranker: Option<String>,
    ) -> Result<TopSnippetsResponse> {
        #[derive(Serialize)]
        struct Request {
            collection_name: String,
            query: String,
            k: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            filter: Option<Filter>,
            #[serde(skip_serializing_if = "Option::is_none")]
            include_document_metadata: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            precise_responses: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            reranker: Option<String>,
        }

        let body = Request {
            collection_name: collection_name.into(),
            query: query.into(),
            k,
            filter,
            include_document_metadata,
            precise_responses,
            reranker,
        };

        self.client.post("/queries/top-snippets", &body).await
    }
}
