//! # ZeroEntropy Rust SDK
//!
//! Rust client library for the [ZeroEntropy API](https://zeroentropy.dev).
//!
//! ## Quick Start
//!
//! ```no_run
//! use zeroentropy_community::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create client from environment variable ZEROENTROPY_API_KEY
//!     let client = Client::from_env()?;
//!
//!     // Create a collection
//!     client.collections().add("my_collection").await?;
//!
//!     // Add a document
//!     client.documents().add_text(
//!         "my_collection",
//!         "doc1.txt",
//!         "This is a test document",
//!         None,
//!     ).await?;
//!
//!     // Search documents
//!     let results = client.queries().top_snippets(
//!         "my_collection",
//!         "test",
//!         10,
//!         None,
//!         None,
//!         None,
//!         None,
//!     ).await?;
//!
//!     println!("Found {} results", results.results.len());
//!     Ok(())
//! }
//! ```

mod client;
mod error;
mod resources;
mod types;

pub use client::{Client, ClientBuilder};
pub use error::{Error, Result};
pub use resources::{Collections, Documents, Models, Queries};
pub use types::*;

impl Client {
    /// Access the collections resource
    pub fn collections(&self) -> Collections {
        Collections::new(self)
    }

    /// Access the documents resource
    pub fn documents(&self) -> Documents {
        Documents::new(self)
    }

    /// Access the queries resource
    pub fn queries(&self) -> Queries {
        Queries::new(self)
    }

    /// Access the models resource
    pub fn models(&self) -> Models {
        Models::new(self)
    }
}
