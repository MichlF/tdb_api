use serde::{Deserialize, Serialize};

/// Represents the Sentiment threshold query parameter
#[derive(Deserialize)]
pub struct SentimentThresholdQuery {
    /// The upper bound of the sentiment threshold.
    pub upper: f64,
    /// The lower bound of the sentiment threshold.
    pub lower: f64,
}

/// Represents the datetime query parameter
#[derive(Deserialize)]
pub struct DateTimeRangeQuery {
    /// The start datetime.
    pub start: String,
    /// The end datetime.
    pub end: String,
}

/// Represents an API response.
#[derive(Serialize)]
pub struct ApiResponse {
    /// The ID of the response.
    pub id: i32,
    /// The date of the response.
    pub date: String,
    /// The URL of the response.
    pub url: String,
    /// The subreddit of the response.
    pub subreddit: String,
    /// The title of the response.
    pub title: String,
    /// The author of the response.
    pub author: String,
    /// The URL contained in the response.
    pub url_contained: String,
    /// The sentiment of the response.
    pub sentiment: f64,
    /// The content of the response.
    pub content: String,
}
