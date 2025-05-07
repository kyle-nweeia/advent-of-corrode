use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

struct FormValues<'a> {
    year: &'a str,
    day: &'a str,
    part: &'a str,
}

impl<'a> FormValues<'a> {
    fn from(vals: &'a std::collections::HashMap<String, FormValue>) -> Option<FormValues<'a>> {
        Some(FormValues {
            year: vals.get("year")?.first()?,
            day: vals.get("day")?.first()?,
            part: vals.get("part")?.first()?,
        })
    }
}

fn app() -> Element {
    let mut answer = use_signal(String::new);

    rsx! {
        form {
            display: "flex",
            flex_direction: "column",
            align_items: "center",
            onsubmit: move |evt: FormEvent| async move {
                if let Some(FormValues { year, day, part }) = FormValues::from(&evt.values()) {
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
