use axum::{routing::post, Json, Router};
use rig::{completion::{CompletionModel, Prompt}, message::Message};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;


#[derive(Deserialize)]
struct PromptRequest {
    prompt: String,
}

#[derive(Serialize)]
struct PromptResponse {
    response: String,
}

async fn handle_prompt<M: CompletionModel>(
    Json(payload): Json<PromptRequest>,
    agent: Arc<rig::agent::Agent<M>>,
) -> Json<PromptResponse> {
    let response = agent
        .prompt(Message::from(payload.prompt))
        .await
        .unwrap_or_else(|_| "Error prompting agent".to_string());

    Json(PromptResponse { response })
}

pub async fn run_server<M: CompletionModel + 'static>(agent: rig::agent::Agent<M>, port: u16, public: bool, https: bool) {
    let agent = Arc::new(agent);

    let app = Router::new().route(
        "/prompt",
        post({
            let agent = Arc::clone(&agent);
            move |payload| handle_prompt(payload, agent.clone())
        }),
    );

    let address = if public { [0, 0, 0, 0] } else { [127, 0, 0, 1] };
    let addr = SocketAddr::from((address, port));

    if https {
        println!("https not implemented");
    } else {

        let listener = TcpListener::bind(addr).await.expect("Failed to bind port");
        println!("Server running on http://{}", addr);

        axum::serve(listener, app).await.expect("Failed to start server");
    }
}