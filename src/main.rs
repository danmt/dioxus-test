#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::io::Write;

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        div {
            header {
                class: "p-4 bg-slate-500/50",
                h1 { 
                    class: "text-blue-500 text-center text-3xl",
                    "Verifyah",
                },
                p {
                    class: "text-center",
                    "Avoid scams, use the cryptography for what it's meant to be.",
                },
            }
            form {
                class: "mx-auto my-4 border border-black p-4 w-[400px]",
                onsubmit: move |e| {
                    async move {
                        match e.data.values().get("url") {
                            Some(url) => {
                                tracing::info!("URL: {:?}", url.as_value());
                                if let Ok(tweet_data) = get_tweet_data(url.as_value()).await {
                                    tracing::info!("Output: {:?}", tweet_data)
                                }
                            },
                            None => tracing::info!("URL not found.")
                        }
                    }
                },
                label {
                    "Enter URL to the tweet:"
                },
                div {
                    class: "flex gap-4",
                    input {
                        class: "border-b border-black px-1/2 pb-1/2 flex-grow",
                        placeholder: "https://x.com/213123",
                        name: "url",
                        r#type: "url",
                    },
                    button {
                        class: "px-2 py-1 bg-slate-500/25 border border-black",
                        r#type: "submit",
                        "Search"
                    },
                },
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawTweetData {
    pub html: String,
    pub author_name: String,
    pub author_url: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetData {
    pub raw_tweet: RawTweetData,
    pub status: TweetStatus, 
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TweetStatus {
    NotVerified,
    Verifying,
    Verified,
}

pub fn get_cwd() -> std::path::PathBuf {
    std::env::current_dir().expect("Shoud be able to read cwd")
}

pub fn write_file(path: &std::path::Path, content: &str) {
    let mut file = std::fs::File::create(path).expect("Should be able to open file");
    file.write_all(content.as_bytes())
        .unwrap_or_else(|_| panic!("Should be able to write file: {path:?}"));
}

#[server(GetServerData)]
async fn get_tweet_data(url: String) -> Result<TweetData, ServerFnError> {
    let cwd = get_cwd();
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
        status: TweetStatus::NotVerified,
    };

    // Store tweet locally.
    let tweet_data_as_string = serde_json::to_string(&tweet_data)?;
    write_file(&tweet_path, &tweet_data_as_string);

    // Return tweet data with status.
    Ok(tweet_data)
}
