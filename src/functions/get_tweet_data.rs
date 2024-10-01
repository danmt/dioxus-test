use dioxus::prelude::*;
use dioxus_logger::tracing;
use reqwest::Url;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RawTweetData {
    pub html: String,
    pub author_name: String,
    pub author_url: String,
    pub url: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TweetData {
    pub raw_tweet: RawTweetData,
    pub status: u8, 
}

#[server(GetServerData)]
pub async fn execute(url: String) -> Result<TweetData, ServerFnError> {
    let cwd = crate::utils::get_cwd();
    let tweets_dir = cwd.join("tweets");

    // Parse the URL to verify it and extract relevant information
    let parsed_url = Url::parse(&url)?;
    let url_domain = parsed_url.domain();
    let url_path_segments = parsed_url.path().split("/");

    // TODO: Verify that the domain is twitter and the path looks like a status path.
    
    // Extract Tweet ID
    let tweet_id = url_path_segments.clone().last().unwrap();

    tracing::info!("tweet id: {:?}", tweet_id);

    // Search for a tweet in the tweets folder. 
    let tweet_path = tweets_dir.join(format!("{}.json", tweet_id));
    let tweet_file = std::fs::read_to_string(&tweet_path);

    // If the tweet is found, we return early.
    if let Ok(tweet_file) = tweet_file {
        let tweet_file_data: TweetData = serde_json::from_str(&tweet_file)?;

        return Ok(tweet_file_data);
    }

    // Get the embed URL.
    let embed_url = Url::parse_with_params("https://publish.twitter.com/oembed", &[("url", url)])?;
    tracing::info!("Embed URL: {}", embed_url);

    // Fetch the tweet's data
    let response = reqwest::get(embed_url).await?;
    tracing::info!("Response: {:?}", response);

    // TODO: Verify response before parsing
    
    // Parse data as tweet data.
    let raw_tweet_data = response.json::<RawTweetData>().await?;
    tracing::info!("Raw tweet data: {:?}", raw_tweet_data);
    
    let tweet_data = TweetData {
        raw_tweet: raw_tweet_data,
        status: 0,
    };

    // Store tweet locally.
    let tweet_data_as_string = serde_json::to_string(&tweet_data)?;
    crate::utils::write_file(&tweet_path, &tweet_data_as_string);

    // Return tweet data with status.
    Ok(tweet_data)
}
