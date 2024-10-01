
pub mod home;
pub mod start_tweet_verification;

pub use home::*;
pub use start_tweet_verification::*;

use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/start_tweet_verification/:url")]
    StartTweetVerification { url: String },
}

