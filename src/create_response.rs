use crate::models::ApiResponse;
use serde_json;

/// Creates an instance of ApiResponse and returns its JSON representation as a string
///
/// # Arguments
///
/// * `row` - A tokio_postgres::Row containing the values to populate the ApiResponse struct
///
/// # Returns
///
/// * A JSON string representation of the ApiResponse struct
pub fn create_response(row: tokio_postgres::Row) -> String {
    // Define the variables
    let id: i32 = row.get(0);
    let date: i64 = row.get(1); // postgresql bigint type (unix timestamp)
    let url: String = row.get(3);
    let subreddit: String = row.get(4);
    let title: String = row.get(5);
    let author: String = row.get(6);
    let url_contained: String = row.get(7);
    let sentiment: f64 = row.get(8);
    let content: String = row.get(9);

    // Convert and format the Unix timestamp to DateTime<Utc> using the from_utc() method
    let date_time = chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp_opt(date, 0).unwrap(),
        chrono::Utc,
    );
    let date = date_time.format("%d-%m-%Y, %H:%M").to_string();

    // Create an instance of ApiResponse to return a JSON representation
    let api_response = ApiResponse {
        id,
        date,
        url,
        subreddit,
        title,
        author,
        url_contained,
        sentiment,
        content,
    };

    // Convert the ApiResponse struct to a JSON string using serde_json::to_string
    serde_json::to_string(&api_response).unwrap()
}
