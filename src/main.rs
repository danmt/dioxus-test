#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;

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
                    "Verifyah"
                }
                p {
                    class: "text-center",
                    "Avoid scams, use the cryptography for what it's meant to be."
                }
            }
            form {
                class: "mx-auto my-4 border border-black p-4 w-[400px]",
                onsubmit: move |e| {
                    async move {
                        match (e.data.values().get("url")) {
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
                }
                div {
                    class: "flex gap-4",
                    input {
                        class: "border-b border-black px-1/2 pb-1/2 flex-grow",
                        placeholder: "https://x.com/213123",
                        name: "url",
                        r#type: "url",
                    }
                    button {
                        class: "px-2 py-1 bg-slate-500/25 border border-black",
                        r#type: "submit",
                        "Search"
                    }               
                }
            }
        }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    tracing::info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_tweet_data(url: String) -> Result<String, ServerFnError> {
    tracing::info!("URL: {}", url);
    // the purpose of this is to fetch a tweet's data from its URL. 
    // 1. given the URL for a real URL to use oEmbed.
    // 2. using the new URL fetch the JSON associated to the tweet.
    // 3. return the tweet data
    Ok("Hello from the server!".to_string())
}
