use axum::{
    body::Body,
    extract::Query,
    http::{
        header::{CONTENT_TYPE, USER_AGENT},
        HeaderMap, Response,
    },
    response::{IntoResponse, Redirect},
    routing::get,
    Router, Server,
};
use html_escape::encode_safe;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    Server::bind(
        &std::env::args()
            .nth(1)
            .expect("Please specify a host address")
            .parse()
            .expect("Failed parsing host address"),
    )
    .serve(
        <Router<Body>>::new()
            .route("/", get(handle))
            .into_make_service(),
    )
    .await
    .unwrap();
}

#[derive(Deserialize)]
struct Embed {
    pass: String,
    title: Option<String>,
    description: Option<String>,
    image: Option<String>,
    #[serde(default)]
    image_large: bool,
}

async fn handle(
    headers: HeaderMap,
    query: Query<Embed>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Some(user_agent) = headers.get(USER_AGENT) {
        if let Ok(user_agent) = user_agent.to_str() {
            if user_agent.contains("Discordbot") {
                println!("Faked Discord embed");
                let mut response = String::new();
                if let Some(title) = &query.title {
                    response.push_str(&format!(
                        r#"<meta property="og:title"content="{}">"#,
                        encode_safe(&title)
                    ));
                }
                if let Some(description) = &query.description {
                    response.push_str(&format!(
                        r#"<meta property="og:description"content="{}">"#,
                        encode_safe(&description)
                    ));
                }
                if let Some(image) = &query.image {
                    response.push_str(&format!(
                        r#"<meta property="og:image"content="{}">"#,
                        encode_safe(&image)
                    ));
                }
                if query.image_large {
                    response
                        .push_str(r#"<meta name="twitter:card" content="summary_large_image">"#);
                }
                return Ok(Response::builder()
                    .header(CONTENT_TYPE, "text/html")
                    .body(Body::from(response))
                    .unwrap());
            }
        }
    }
    Err(Redirect::permanent(&query.pass))
}
