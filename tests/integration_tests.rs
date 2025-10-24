use zeroentropy_community::{Client, DocumentContent, MetadataValue};
use std::collections::HashMap;

#[test]
fn test_client_creation() {
    // Test creating client with explicit API key
    let client = Client::new("test-api-key");
    assert!(client.is_ok());
}

#[test]
fn test_client_builder() {
    use std::time::Duration;
    
    let client = Client::builder()
        .api_key("test-key")
        .timeout(Duration::from_secs(30))
        .max_retries(5)
        .build();
    
    assert!(client.is_ok());
}

#[test]
fn test_document_content_text() {
    let content = DocumentContent::Text {
        text: "Test content".to_string(),
    };
    
    // Serialize to JSON
    let json = serde_json::to_string(&content).unwrap();
    assert!(json.contains("\"type\":\"text\""));
    assert!(json.contains("\"text\":\"Test content\""));
}

#[test]
fn test_document_content_auto() {
    let content = DocumentContent::Auto {
        base64_data: "dGVzdA==".to_string(),
    };
    
    // Serialize to JSON
    let json = serde_json::to_string(&content).unwrap();
    assert!(json.contains("\"type\":\"auto\""));
    assert!(json.contains("\"base64_data\":\"dGVzdA==\""));
}

#[test]
fn test_metadata_value_string() {
    let value = MetadataValue::String("test".to_string());
    let json = serde_json::to_string(&value).unwrap();
    assert_eq!(json, "\"test\"");
}

#[test]
fn test_metadata_value_array() {
    let value = MetadataValue::Array(vec!["a".to_string(), "b".to_string()]);
    let json = serde_json::to_string(&value).unwrap();
    assert_eq!(json, "[\"a\",\"b\"]");
}

#[test]
fn test_metadata_serialization() {
    let mut metadata = HashMap::new();
    metadata.insert(
        "category".to_string(),
        MetadataValue::String("test".to_string()),
    );
    metadata.insert(
        "tags".to_string(),
        MetadataValue::Array(vec!["rust".to_string(), "sdk".to_string()]),
    );
    
    let json = serde_json::to_string(&metadata).unwrap();
    assert!(json.contains("category"));
    assert!(json.contains("tags"));
}

#[test]
fn test_error_display() {
    use zeroentropy_community::Error;
    
    let err = Error::NotFound("Collection not found".to_string());
    assert_eq!(err.to_string(), "Not found: Collection not found");
    
    let err = Error::Conflict("Resource already exists".to_string());
    assert_eq!(err.to_string(), "Conflict: Resource already exists");
}

#[test]
fn test_latency_mode_serialization() {
    use zeroentropy_community::LatencyMode;
    
    let low = LatencyMode::Low;
    assert_eq!(serde_json::to_string(&low).unwrap(), "\"low\"");
    
    let high = LatencyMode::High;
    assert_eq!(serde_json::to_string(&high).unwrap(), "\"high\"");
}

#[test]
fn test_index_status_serialization() {
    use zeroentropy_community::IndexStatus;
    
    let status = IndexStatus::Indexed;
    assert_eq!(serde_json::to_string(&status).unwrap(), "\"indexed\"");
    
    let status = IndexStatus::ParsingFailed;
    assert_eq!(serde_json::to_string(&status).unwrap(), "\"parsing_failed\"");
}

// Note: Integration tests that require actual API calls should be run separately
// with a valid API key and can be placed in a separate test file that's ignored by default
