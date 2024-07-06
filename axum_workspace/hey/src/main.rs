use axum::{
    routing::{get, post},
    Router,
    extract::{Query, State},
    response::{Redirect, Html},
};
use axum::http::StatusCode;
use tokio::fs;


use serde::{Deserialize, Serialize};

use serde_json::from_str;


use tokio::sync::Mutex;

use async_session::Session;

use std::collections::HashMap;
use std::sync::Arc;

struct AppState {
    session: Mutex<Session>,
    config: Config,
}

struct XToken {

}


#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    #[serde(flatten)]
    pub values: HashMap<String, String>,
}

async fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_file_path = "./config.json";
    let contents = fs::read_to_string(config_file_path).await?;
    let config: Config = from_str(&contents)?;
    Ok(config)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config().await?;
    println!("Config: {:?}", config);

    let state = Arc::new(AppState {
        session: Mutex::new(Session::new()),
        config,
    });

    let app = Router::new()
        .route("/", get(get_home))
        .route("/x_auth", get(x_auth_user))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/callback", get(callback))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn login(State(state): State<Arc<AppState>>) -> Redirect {
    let mut session = state.session.lock().await;
    session.insert("is_authenticated", true).unwrap();
    println!("Logged in!");
    Redirect::to("/")
}


async fn logout(State(state): State<Arc<AppState>>) -> Redirect {
    let mut session = state.session.lock().await;
    
    // Remove authentication status
    let _ = session.remove("is_authenticated");
    // Or to clear entire session: session.clear();
    
    println!("Logged out!");
    
    // Redirect to home page
    Redirect::to("/")
}

async fn get_home(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let mut session = state.session.lock().await;
    let is_authenticated = session.get::<bool>("is_authenticated").unwrap_or(false);

    let body = if is_authenticated {
        println!("Authenticated user");
        format!(
            r#"
            <h1>Hey, authenticated user!</h1>
            <form action="/logout" method="post">
                <button type="submit">Logout</button>
            </form>
            "#
        )
    } else {
        println!("Unauthenticated user");
        format!(
            r#"
            <h1>Welcome to the landing page!</h1>
            <form action="/login" method="post">
                <button type="submit">Login</button>
            </form>
            "#
        )
    };
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Home Page</title>
        </head>
        <body>
            {}
        </body>
        </html>
        "#,
        body
    );
    Ok(Html(html))
}

async fn x_auth_user(State(state): State<Arc<AppState>>) -> Result<Redirect, StatusCode> {
    let x_token = make_x_token(&state.config);
    // Implement the rest of the x_auth_user logic here
    // ...

    Ok(Redirect::to("/callback")) // Replace with actual redirect URL
}

async fn callback(
    Query(params): Query<std::collections::HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> Result<Redirect, StatusCode> {
    let code = params.get("code").ok_or(StatusCode::UNPROCESSABLE_ENTITY)?;
    // Implement the rest of the callback logic here
    // ...

    Ok(Redirect::to("/")) // Redirect to home after successful auth
}

// Implement this function
fn make_x_token(config: &Config) -> XToken {
    // ...
    XToken {}
}
