
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_http::services::ServeDir;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Query, State, WebSocketUpgrade,
    },
    response::{Html, IntoResponse},
    routing::{get, get_service, post},
    Json, Router,
};

#[tokio::main]
async fn main() {
    let mut backend = Backend::new();
    backend.app.route(
        "/api/mes",
        post(handle_message).with_state(backend.messages.clone())
    )
    .route("/api/get_mes", get(get_mes).with_state(backend.messages));


    backend.start_app();




    
}

async fn serve_html() -> Html<String> {
    let html = std::fs::read_to_string("../chis_frontend/index.html").unwrap();
    Html(html)
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}
async fn handle_socket(mut socket: WebSocket) {
    socket.send(Message::Ping(vec![1, 2, 3])).await.unwrap();
    println!("{:?}", socket);
}

#[derive(Deserialize, Serialize, Clone)]
struct MessageData {
    m_text: String,
}

impl MessageData {
    fn new(text: String) -> Self {
        MessageData { m_text: text }
    }
}

async fn handle_message(
    mut messages: State<Vec<MessageData>>,
    Json(payload): Json<MessageData>,
) -> Json<String> {
    messages.push(MessageData::new(payload.m_text));

    Json(format!("Received"))
}

async fn get_mes(messages: State<Vec<MessageData>>) -> Json<Vec<MessageData>> {
    Json(messages.to_vec())
}

struct Backend {
    app: Router,
    messages: Vec<MessageData>,
}

impl Backend {
    fn new() -> Self {
        Backend {
            app: Router::new().nest_service(
                "/static",
                get_service(ServeDir::new("../chis_frontend/dist")),
            ).route("/", get(serve_html))
            ,
            messages: vec![],
        }
    }

    async fn start_app(self){
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
        axum::serve(listener, self.app).await.unwrap();
    }

}
