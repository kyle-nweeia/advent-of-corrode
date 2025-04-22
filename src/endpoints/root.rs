use axum::{response::Html, routing::Router};

async fn handler() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head>
                <script>
                    function setAction(form) {
                        form.action = `/submit/${form.year.value}/${form.day.value}/${form.part.value}`;
                    }
                </script>
            </head>
            <body>
                <form>
                    <div>
                        <label>Year:</label>
                        <input name="year" onchange="setAction(form)" type="text">
                    </div>
                    <div>
                        <label>Day:</label>
                        <input name="day" onchange="setAction(form)" type="text">
                    </div>
                    <div>
                        <label>Part:</label>
                        <input name="part" onchange="setAction(form)" type="text">
                    </div>
                    <div>
                        <input type="submit">
                    </div>
                </form>
            </body>
        </html>
        "#,
    )
}

pub fn router() -> Router {
    Router::new().route("/", axum::routing::get(handler))
}
