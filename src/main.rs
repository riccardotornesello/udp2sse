mod broadcaster;
mod listener;

use self::broadcaster::Broadcaster;
use crate::listener::listener_handler;
use actix_web::{web, App, HttpServer, Responder};
use std::sync::Arc;

const PING_INTERVAL: u64 = 20;
const SSE_ADDRESS: &str = "0.0.0.0:8000";
const LISTENER_ADDRESS: &str = "0.0.0.0:34254";

pub struct AppState {
    broadcaster: Arc<Broadcaster>,
}

pub async fn new_sse_client(state: web::Data<AppState>) -> impl Responder {
    state.broadcaster.new_client().await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let broadcaster: Arc<Broadcaster> = Broadcaster::create(PING_INTERVAL);

    actix_web::rt::spawn(listener_handler(LISTENER_ADDRESS, broadcaster.clone()));

    println!("Serving SSE on http://{}/events", SSE_ADDRESS);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                broadcaster: Arc::clone(&broadcaster),
            }))
            .route("/events", web::get().to(new_sse_client))
    })
    .bind(SSE_ADDRESS)?
    .disable_signals()
    .run()
    .await
}
