use dioxus::prelude::*;
use dioxus_logger::tracing;

#[component]
pub fn Home() -> Element {
    let mut tweet_url = use_signal(|| String::from(""));
    let nav = navigator();

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
                onsubmit: move |_| {
                    tracing::info!("current url: {}", tweet_url());

                    nav.replace(crate::pages::Route::StartTweetVerification { url: tweet_url() });
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
                        oninput: move |event| tweet_url.set(event.value())
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

