use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, VecDeque},
    net::SocketAddr,
    sync::Arc,
};
use tokio::sync::{mpsc, Mutex};
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
enum ServerMessage {
    #[serde(rename = "status")]
    Status { status: String },
    #[serde(rename = "chat")]
    ChatMessage { sender: String, text: String },
    #[serde(rename = "system")]
    SystemMessage { text: String },
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum ClientMessage {
    #[serde(rename = "send_chat")]
    SendChat { text: String },
    #[serde(rename = "next")]
    Next,
}

struct AppState {
    users: HashMap<Uuid, mpsc::UnboundedSender<ServerMessage>>,
    waiting_queue: VecDeque<Uuid>,
    matches: HashMap<Uuid, Uuid>,
}

impl AppState {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
            waiting_queue: VecDeque::new(),
            matches: HashMap::new(),
        }
    }
}

type SharedState = Arc<Mutex<AppState>>;

async fn matchmake(user_id: Uuid, state: &SharedState) {
    let mut s = state.lock().await;

    // If the user is already matched, do nothing.
    if s.matches.contains_key(&user_id) {
        return;
    }

    // Find a partner in the waiting queue who is not the user itself and is still connected.
    let mut partner_id = None;
    while let Some(peer_id) = s.waiting_queue.pop_front() {
        if peer_id != user_id && s.users.contains_key(&peer_id) {
            partner_id = Some(peer_id);
            break;
        }
    }

    if let Some(peer_id) = partner_id {
        // Match them!
        s.matches.insert(user_id, peer_id);
        s.matches.insert(peer_id, user_id);

        // Notify both users
        if let Some(sender) = s.users.get(&user_id) {
            let _ = sender.send(ServerMessage::Status {
                status: "matched".to_string(),
            });
            let _ = sender.send(ServerMessage::SystemMessage {
                text: "You are now chatting with a random stranger!".to_string(),
            });
        }
        if let Some(sender) = s.users.get(&peer_id) {
            let _ = sender.send(ServerMessage::Status {
                status: "matched".to_string(),
            });
            let _ = sender.send(ServerMessage::SystemMessage {
                text: "You are now chatting with a random stranger!".to_string(),
            });
        }
        println!("Matched user {} with user {}", user_id, peer_id);
    } else {
        // Nobody available, push user to waiting queue (if they aren't already there)
        if !s.waiting_queue.contains(&user_id) {
            s.waiting_queue.push_back(user_id);
        }
        if let Some(sender) = s.users.get(&user_id) {
            let _ = sender.send(ServerMessage::Status {
                status: "waiting".to_string(),
            });
            let _ = sender.send(ServerMessage::SystemMessage {
                text: "Looking for a partner...".to_string(),
            });
        }
        println!("User {} is waiting in queue", user_id);
    }
}

async fn disconnect_user(user_id: Uuid, state: &SharedState) {
    let mut s = state.lock().await;

    // Remove from active users list
    s.users.remove(&user_id);

    // Remove from waiting queue if they are there
    s.waiting_queue.retain(|&id| id != user_id);

    // If they were matched, disconnect the partner
    if let Some(partner_id) = s.matches.remove(&user_id) {
        s.matches.remove(&partner_id);

        if let Some(sender) = s.users.get(&partner_id) {
            let _ = sender.send(ServerMessage::Status {
                status: "partner_disconnected".to_string(),
            });
            let _ = sender.send(ServerMessage::SystemMessage {
                text: "Stranger has disconnected. Click 'Next' to find someone new.".to_string(),
            });
        }
        println!("User {} disconnected; partner {} was notified", user_id, partner_id);
    } else {
        println!("User {} disconnected while waiting or idle", user_id);
    }
}

async fn skip_user(user_id: Uuid, state: &SharedState) {
    let mut s = state.lock().await;

    // If matched, notify and disconnect partner
    if let Some(partner_id) = s.matches.remove(&user_id) {
        s.matches.remove(&partner_id);

        if let Some(sender) = s.users.get(&partner_id) {
            let _ = sender.send(ServerMessage::Status {
                status: "partner_disconnected".to_string(),
            });
            let _ = sender.send(ServerMessage::SystemMessage {
                text: "Stranger has left the chat. Click 'Next' to find someone new.".to_string(),
            });
        }
        println!("User {} skipped; partner {} was notified", user_id, partner_id);
    }

    // Remove from waiting queue if somehow in it
    s.waiting_queue.retain(|&id| id != user_id);

    // Drop the lock before running matchmaking to avoid deadlock
    drop(s);

    matchmake(user_id, state).await;
}

async fn send_chat(from_id: Uuid, text: String, state: &SharedState) {
    let s = state.lock().await;

    if let Some(&partner_id) = s.matches.get(&from_id) {
        // Send to partner as "stranger"
        if let Some(sender) = s.users.get(&partner_id) {
            let _ = sender.send(ServerMessage::ChatMessage {
                sender: "stranger".to_string(),
                text: text.clone(),
            });
        }
        // Echo to self as "me"
        if let Some(sender) = s.users.get(&from_id) {
            let _ = sender.send(ServerMessage::ChatMessage {
                sender: "me".to_string(),
                text,
            });
        }
    } else {
        // Not matched, maybe send a system message
        if let Some(sender) = s.users.get(&from_id) {
            let _ = sender.send(ServerMessage::SystemMessage {
                text: "You are not matched with anyone yet!".to_string(),
            });
        }
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: SharedState) {
    let (mut sender, mut receiver) = socket.split();
    let user_id = Uuid::new_v4();
    let (tx, mut rx) = mpsc::unbounded_channel::<ServerMessage>();

    // Register user
    {
        let mut s = state.lock().await;
        s.users.insert(user_id, tx);
        println!("New WebSocket connection: user {}", user_id);
    }

    // Automatically matchmake on connection
    matchmake(user_id, &state).await;

    // Spawn task to send outgoing messages to the WebSocket
    let mut send_task = tokio::spawn({
        let user_id = user_id;
        async move {
            while let Some(msg) = rx.recv().await {
                if let Ok(text) = serde_json::to_string(&msg) {
                    if sender.send(Message::Text(text)).await.is_err() {
                        break;
                    }
                }
            }
            println!("Send task for user {} finished", user_id);
        }
    });

    // Read incoming messages from WebSocket
    let state_clone = state.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                match client_msg {
                    ClientMessage::SendChat { text } => {
                        send_chat(user_id, text, &state_clone).await;
                    }
                    ClientMessage::Next => {
                        skip_user(user_id, &state_clone).await;
                    }
                }
            }
        }
        println!("Receive task for user {} finished", user_id);
    });

    // Wait for either task to finish (e.g. socket disconnected or receiver closed)
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };

    // Clean up user
    disconnect_user(user_id, &state).await;
}

#[tokio::main]
async fn main() {
    // Initialize tracing logger
    tracing_subscriber::fmt::init();

    let state = Arc::new(Mutex::new(AppState::new()));

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Find the frontend/dist folder
    let mut frontend_dir = std::env::current_dir().unwrap_or_default();
    if frontend_dir.join("../frontend/dist").exists() {
        frontend_dir = frontend_dir.join("../frontend/dist");
    } else {
        frontend_dir = frontend_dir.join("frontend/dist");
    }

    let fallback_path = frontend_dir.join("index.html");
    println!("Static files served from: {}", frontend_dir.display());

    use tower_http::services::{ServeDir, ServeFile};

    // Define routing
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .nest_service("/", ServeDir::new(&frontend_dir).fallback(ServeFile::new(&fallback_path)))
        .layer(cors)
        .with_state(state);

    // Listen address
    let port = 3000;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Backend server running on http://localhost:{}", port);

    axum::serve(listener, app).await.unwrap();
}
