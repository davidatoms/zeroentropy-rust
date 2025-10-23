use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Document content types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DocumentContent {
    /// Plain text content
    Text { text: String },
    /// Auto-detect format (for PDFs, images with OCR)
    Auto { base64_data: String },
}

/// Metadata type for documents
pub type Metadata = HashMap<String, MetadataValue>;

/// Metadata values can be strings or arrays of strings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MetadataValue {
    String(String),
    Array(Vec<String>),
}

/// Filter for querying documents
pub type Filter = HashMap<String, serde_json::Value>;

/// Latency mode for queries
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LatencyMode {
    Low,
    High,
}

/// Index status for documents
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndexStatus {
    NotParsed,
    NotIndexed,
    Parsing,
    ParsingFailed,
    Indexing,
    IndexingFailed,
    Indexed,
}

/// Response from status endpoint
#[derive(Debug, Deserialize)]
pub struct StatusResponse {
    pub num_documents: u64,
    pub num_collections: u64,
}

/// Response from collection add/delete
#[derive(Debug, Deserialize)]
pub struct CollectionResponse {
    pub message: String,
}

/// Response from get collection list
#[derive(Debug, Deserialize)]
pub struct CollectionListResponse {
    pub collections: Vec<String>,
}

/// Response from document add/update/delete
#[derive(Debug, Deserialize)]
pub struct DocumentResponse {
    pub message: String,
}

/// Document information
#[derive(Debug, Deserialize)]
pub struct DocumentInfo {
    pub path: String,
    pub index_status: IndexStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<DocumentContent>,
}

/// Response from get document info
#[derive(Debug, Deserialize)]
pub struct DocumentInfoResponse {
    pub document: DocumentInfo,
}

/// Response from get document info list
#[derive(Debug, Deserialize)]
pub struct DocumentInfoListResponse {
    pub documents: Vec<DocumentInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_gt: Option<String>,
}

/// Page information
#[derive(Debug, Deserialize)]
pub struct PageInfo {
    pub path: String,
    pub page_number: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// Response from get page info
#[derive(Debug, Deserialize)]
pub struct PageInfoResponse {
    pub page: PageInfo,
}

/// Query result for top documents
#[derive(Debug, Deserialize)]
pub struct DocumentResult {
    pub path: String,
    pub score: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Response from top documents query
#[derive(Debug, Deserialize)]
pub struct TopDocumentsResponse {
    pub results: Vec<DocumentResult>,
}

/// Query result for top pages
#[derive(Debug, Deserialize)]
pub struct PageResult {
    pub path: String,
    pub page_number: u32,
    pub score: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// Response from top pages query
#[derive(Debug, Deserialize)]
pub struct TopPagesResponse {
    pub results: Vec<PageResult>,
}

/// Query result for top snippets
#[derive(Debug, Deserialize)]
pub struct SnippetResult {
    pub path: String,
    pub content: String,
    pub score: f64,
    pub page_number: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Response from top snippets query
#[derive(Debug, Deserialize)]
pub struct TopSnippetsResponse {
    pub results: Vec<SnippetResult>,
}

/// Document for reranking
#[derive(Debug, Serialize)]
pub struct RerankDocument {
    pub id: String,
    pub text: String,
}

/// Rerank result
#[derive(Debug, Deserialize)]
pub struct RerankResult {
    pub id: String,
    pub score: f64,
    pub index: usize,
}

/// Response from rerank endpoint
#[derive(Debug, Deserialize)]
pub struct RerankResponse {
    pub results: Vec<RerankResult>,
}
