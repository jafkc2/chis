use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    response::Html,
    routing::{get, get_service, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

#[tokio::main]
async fn main() {
    let messages: Vec<Message> = vec![];
    let shared_state = Arc::new(Mutex::new(messages));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app: Router = Router::new()
        .nest_service(
            "/static",
            get_service(ServeDir::new("../chis_frontend/dist")),
        )
        .route("/", get(serve_html))
        .route("/api/send_message", post(receive_message_handler))
        .route("/api/get_messages", get(get_messages_handler))
        .with_state(shared_state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn serve_html() -> Html<String> {
    let html = std::include_str!("../../chis_frontend/index.html").to_string();
    Html(html)
}

async fn receive_message_handler(
    State(messages): State<Arc<Mutex<Vec<Message>>>>,
    Json(payload): Json<Message>,
) -> Json<String> {
    let mut locked_messages = messages.lock().unwrap();
    locked_messages.push(Message {
        author: payload.author,
        text: payload.text,
    });

    Json(String::from("Received."))
}

async fn get_messages_handler(
    State(messages): State<Arc<Mutex<Vec<Message>>>>,
) -> Json<Vec<Message>> {
    Json(messages.lock().unwrap().to_vec())
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Message {
    author: String,
    text: String,
}
