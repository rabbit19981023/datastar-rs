use axum::{Router, routing::get};
use maud::{Markup, html};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!(
        "server is listening on {}...",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> Markup {
    html! {
        h1 { "Hello, world!" }
    }
}
