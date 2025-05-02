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
            onsubmit: move |evt: FormEvent| async move {
                if let Some(FormValues { year, day, part }) = FormValues::from(&evt.values()) {
                    let result = reqwest::Client::new()
                        .get(format!("http://localhost:3000/submit/{year}/{day}/{part}"))
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
            div { "Answer: {answer}" }
        }
    }
}
