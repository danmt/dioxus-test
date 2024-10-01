use dioxus::prelude::*;
use dioxus_logger::tracing;

#[derive(PartialEq, Props, Clone)]
struct StartTweetVerificationDetailsProps {
    pub html: String,
}

fn StartTweetVerificationDetails(props: StartTweetVerificationDetailsProps) -> Element {
    rsx! {
        div {
            dangerous_inner_html: "{props.html}"
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct StartTweetVerificationActionsProps {
    pub status: u8,
}

fn StartTweetVerificationActions(props: StartTweetVerificationActionsProps) -> Element {
    rsx! {
        div {
            class: "border border-black p-4",
            match props.status {
                0 => rsx! {
                    p { "Not Verified" },
                    button {
                        class: "px-2 py-1 bg-slate-500/25 border border-black",
                        "Start Verification"
                    },
                },
                1 => rsx! {
                    p { "Verifying" }
                },
                2 => rsx! {
                    p { "Verified" }
                },
                _ => rsx! {
                    "Error"
                }
            }
        }
    }
}

#[component]
pub fn StartTweetVerification(url: String) -> Element {
    tracing::info!("tweet_url: {}", url);

    let mut my_tweet = use_resource(move || {
        let url_copy = url.clone();

        async move {
            crate::functions::get_tweet_data::execute(url_copy).await
        }   
    });

    match &*my_tweet.read_unchecked() {
        Some(Ok(response)) =>
            rsx! {
                div {
                    class: "flex gap-2",

                    StartTweetVerificationDetails {
                        html: "{response.raw_tweet.html}"
                    },

                    StartTweetVerificationActions {
                        status: response.status
                    }
                }
            },
        Some(Err(_)) => rsx! { div { "Loading tweet failed" } },
        None => rsx! { div { "Loading tweet..." } },
    }
}

