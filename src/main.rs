use std::sync::Arc;

use axum::{extract::State, http::StatusCode, routing::get, Router};
use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::{Resend, Result};

// Cloning the Resend client is fine and cheap as the internal HTTP client is
// not cloned.
#[derive(Clone)]
struct AppState {
    resend: Resend,
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(AppState {
        resend: Resend::new("re_123456789"),
    });

    // build our application with a single route
    let app = Router::new()
        .route("/", get(endpoint))
        // provide the state so the router can access it
        .with_state(shared_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn endpoint(State(state): State<Arc<AppState>>) -> Result<String, StatusCode> {
    let from = "Acme <onboarding@resend.dev>";
    let to = ["delivered@resend.dev"];
    let subject = "Hello World";

    let email =
        CreateEmailBaseOptions::new(from, to, subject).with_html("<strong>It works!</strong>");

    // access the state via the `State` extractor and handle the error
    match state.resend.emails.send(email).await {
        Ok(email) => Ok(email.id.to_string()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
