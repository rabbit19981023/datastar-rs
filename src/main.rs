use axum::{
    Router,
    response::{Sse, sse::Event},
    routing::get,
};
use futures_util::{Stream, StreamExt, stream};
use maud::{Markup, html};
use std::{convert::Infallible, time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/hal-html", get(get_hal_html))
        .route("/hal-sse", get(get_hal_sse));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!(
        "server is listening on {}...",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Markup {
    html! {
        button data-on:click="@get('/hal-html')" {
            "Open the pod bay doors, HAL. (text/html)"
        }

        button data-on:click="@get('/hal-sse')" {
            "Open the pod bay doors, HAL. (text/event-stream)"
        }

        div id="hal-html" {}
        div id="hal-sse" {}

        script type="module" src="https://cdn.jsdelivr.net/gh/starfederation/datastar@1.0.0-RC.7/bundles/datastar.js" {}
    }
}

async fn get_hal_html() -> Markup {
    html! {
        div id="hal-html" { "I’m sorry, Dave. I’m afraid I can’t do that. (text/html)" }
    }
}

async fn get_hal_sse() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let repeat = 5;
    let delay = 800;

    let stream = stream::iter(0..repeat).then(move |i| async move {
        let patch = html! {
            div id="hal-sse" { "Times: " (i) ". I’m sorry, Dave. I’m afraid I can’t do that. (text/event-stream)" }
        };
        let event = Event::default()
            .event("datastar-patch-elements")
            .data(format!("elements {}", patch.into_string()));

        sleep(Duration::from_millis(delay)).await;

        Ok(event)
    });

    Sse::new(stream)
}
