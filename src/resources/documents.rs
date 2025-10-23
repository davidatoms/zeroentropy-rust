use crate::client::Client;
use crate::error::Result;
use crate::types::{
    DocumentContent, DocumentInfoListResponse, DocumentInfoResponse, DocumentResponse,
    IndexStatus, Metadata, PageInfoResponse,
};
use serde::Serialize;

/// Documents resource for managing documents in collections
pub struct Documents<'a> {
    client: &'a Client,
}

impl<'a> Documents<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Add a document to a collection
    ///
    /// # Arguments
    /// * `collection_name` - Name of the collection
    /// * `path` - Document path/identifier
    /// * `content` - Document content
    /// * `metadata` - Optional metadata
    /// * `overwrite` - Whether to overwrite if exists
    pub async fn add(
        &self,
        collection_name: impl Into<String>,
        path: impl Into<String>,
        content: DocumentContent,
        metadata: Option<Metadata>,
        overwrite: Option<bool>,
    ) -> Result<DocumentResponse> {
        #[derive(Serialize)]
        struct Request {
            collection_name: String,
            path: String,
            content: DocumentContent,
            #[serde(skip_serializing_if = "Option::is_none")]
            metadata: Option<Metadata>,
            #[serde(skip_serializing_if = "Option::is_none")]
            overwrite: Option<bool>,
        }

        let body = Request {
            collection_name: collection_name.into(),
            path: path.into(),
            content,
            metadata,
            overwrite,
        };

        self.client.post("/documents/add-document", &body).await
    }

    /// Add a text document
    ///
    /// Convenience method for adding plain text documents
    pub async fn add_text(
        &self,
        collection_name: impl Into<String>,
        path: impl Into<String>,
        text: impl Into<String>,
        metadata: Option<Metadata>,
    ) -> Result<DocumentResponse> {
        let content = DocumentContent::Text {
            text: text.into(),
        };
        self.add(collection_name, path, content, metadata, None).await
    }

    /// Add a PDF document from base64 data
    ///
    /// Convenience method for adding PDF documents with OCR
    pub async fn add_pdf(
        &self,
        collection_name: impl Into<String>,
        path: impl Into<String>,
        base64_data: impl Into<String>,
        metadata: Option<Metadata>,
    ) -> Result<DocumentResponse> {
        let content = DocumentContent::Auto {
            base64_data: base64_data.into(),
        };
        self.add(collection_name, path, content, metadata, None).await
    }

    /// Add a PDF document from file path
    ///
    /// Reads the file and encodes it as base64
    pub async fn add_pdf_file(
        &self,
        collection_name: impl Into<String>,
        document_path: impl Into<String>,
        file_path: impl AsRef<std::path::Path>,
        metadata: Option<Metadata>,
    ) -> Result<DocumentResponse> {
        use base64::{engine::general_purpose, Engine as _};
        
        let bytes = tokio::fs::read(file_path).await?;
        let base64_data = general_purpose::STANDARD.encode(&bytes);
        
        self.add_pdf(collection_name, document_path, base64_data, metadata).await
    }

    /// Update a document's metadata or index status
    pub async fn update(
        &self,
        collection_name: impl Into<String>,
        path: impl Into<String>,
        metadata: Option<Metadata>,
        index_status: Option<IndexStatus>,
    ) -> Result<DocumentResponse> {
        #[derive(Serialize)]
        struct Request {
            collection_name: String,
            path: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            metadata: Option<Metadata>,
            #[serde(skip_serializing_if = "Option::is_none")]
            index_status: Option<IndexStatus>,
        }

        let body = Request {
            collection_name: collection_name.into(),
            path: path.into(),
            metadata,
            index_status,
        };

        self.client.post("/documents/update-document", &body).await
    }

    /// Delete a document
    pub async fn delete(
        &self,
        collection_name: impl Into<String>,
        path: impl Into<String>,
    ) -> Result<DocumentResponse> {
        #[derive(Serialize)]
        struct Request {
            collection_name: String,
            path: String,
        }

        let body = Request {
            collection_name: collection_name.into(),
            path: path.into(),
        };

        self.client.post("/documents/delete-document", &body).await
    }

    /// Get document information
    pub async fn get_info(
        &self,
        collection_name: impl Into<String>,
        path: impl Into<String>,
        include_content: Option<bool>,
    ) -> Result<DocumentInfoResponse> {
        #[derive(Serialize)]
        struct Request {
            collection_name: String,
            path: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            include_content: Option<bool>,
        }

        let body = Request {
            collection_name: collection_name.into(),
            path: path.into(),
            include_content,
        };

        self.client.post("/documents/get-document-info", &body).await
    }

    /// Get list of documents in a collection
    pub async fn get_info_list(
        &self,
        collection_name: impl Into<String>,
        limit: Option<u32>,
        path_gt: Option<String>,
    ) -> Result<DocumentInfoListResponse> {
        #[derive(Serialize)]
        struct Request {
            collection_name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            path_gt: Option<String>,
        }

        let body = Request {
            collection_name: collection_name.into(),
            limit,
            path_gt,
        };

        self.client.post("/documents/get-document-info-list", &body).await
    }

    /// Get information about a specific page
    pub async fn get_page_info(
        &self,
        collection_name: impl Into<String>,
        path: impl Into<String>,
        page_number: u32,
        include_content: Option<bool>,
    ) -> Result<PageInfoResponse> {
        #[derive(Serialize)]
        struct Request {
            collection_name: String,
            path: String,
            page_number: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            include_content: Option<bool>,
        }

        let body = Request {
            collection_name: collection_name.into(),
            path: path.into(),
            page_number,
            include_content,
        };

        self.client.post("/documents/get-page-info", &body).await
    }
}
