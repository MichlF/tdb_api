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

    // Define the API response
    format!("Post ID: {}, from {} with reddit-url {} in subreddit {} with title {} by author {} contains the url {} with a sentiment score of {}\n Here is the text/content: {}", id, date, url, subreddit, title, author, url_contained, sentiment, content)
}