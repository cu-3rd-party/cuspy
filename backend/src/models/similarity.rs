use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
pub struct SimilarityRequest {
    pub left: Value,
    pub right: Value,
}

#[derive(Serialize)]
pub struct SimilarityResponse {
    pub similarity_score: f64,
    pub matching_keys: Vec<String>,
    pub differing_keys: Vec<String>,
    pub left_only_keys: Vec<String>,
    pub right_only_keys: Vec<String>,
}
