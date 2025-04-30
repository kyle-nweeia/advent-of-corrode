use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        form {
            onsubmit: async |evt: FormEvent| {
                let year = evt.values()["year"].as_value();
                let day = evt.values()["day"].as_value();
                let part = evt.values()["part"].as_value();
                let result = reqwest::Client::new()
                    .get(format!("http://localhost:3000/submit/{year}/{day}/{part}"))
                    .send()
                    .await;
                if let Ok(response) = result {
                    if let Ok(text) = response.text().await {
                        dioxus_logger::tracing::info!("{text:?}");
                    }
                }
            },

            div {
                label { "Year:" }
                input { name: "year" }
            }
            div {
                label { "Day:" }
                input { name: "day" }
            }
            div {
                label { "Part:" }
                input { name: "part" }
            }
            div {
                input { type: "submit" }
            }
        }
    }
}
