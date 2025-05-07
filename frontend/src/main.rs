use dioxus::prelude::*;
use types::Solution;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let mut answer = use_signal(String::new);

    rsx! {
        form {
            display: "flex",
            flex_direction: "column",
            align_items: "center",
            onsubmit: move |evt: FormEvent| async move {
                if let Some(Solution { year, day, part }) = Solution::from(&evt.values()) {
                    let result = reqwest::Client::new()
                        .get(format!("http://localhost:3000/solution/{year}/{day}/{part}"))
                        .send()
                        .await;
                    if let Ok(response) = result {
                        if let Ok(text) = response.text().await {
                            answer.set(text);
                        }
                    }
                }
            },

            div {
                label { margin: "5px", "Year:" }
                input { margin: "5px", name: "year" }
            }
            div {
                label { margin: "5px", "Day:" }
                input { margin: "5px", name: "day" }
            }
            div {
                label { margin: "5px", "Part:" }
                input { margin: "5px", name: "part" }
            }
            div {
                input { margin: "5px", r#type: "submit" }
            }
            div { margin: "5px", "Answer:" }
            div { margin: "5px", "{answer}" }
        }
    }
}
