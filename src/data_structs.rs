use serde::Deserialize;

// Define a struct to represent the Sentiment threshold query parameter
#[derive(Deserialize)]
pub struct SentimentThresholdQuery {
    pub upper: f64,
    pub lower: f64,
}

// Define a struct to represent the datetime query parameter
#[derive(Deserialize)]
pub struct DateTimeRangeQuery {
    pub start: String,
    pub end: String,
}