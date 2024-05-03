use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use reqwest::Client as HttpClient;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::sync::Mutex as AsyncMutex;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ForexPair {
    id: u64,
    pair: String,
    price: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    forex_pairs: HashMap<u64, ForexPair>,
}

impl Database {
    fn new() -> Self {
        Self {
            forex_pairs: HashMap::new(),
        }
    }

    fn insert(&mut self, forex_pair: ForexPair) {
        self.forex_pairs.insert(forex_pair.id, forex_pair);
    }

    fn get(&self, id: &u64) -> Option<&ForexPair> {
        self.forex_pairs.get(id)
    }

    fn get_all(&self) -> Vec<&ForexPair> {
        self.forex_pairs.values().collect()
    }
}

struct AppState {
    db: Arc<AsyncMutex<Database>>,
}

async fn get_forex_price(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db = app_state.db.lock().await;
    match db.get(&id.into_inner()) {
        Some(forex_pair) => HttpResponse::Ok().json(forex_pair),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn get_all_forex_prices(app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.db.lock().await;
    let forex_pairs = db.get_all();
    HttpResponse::Ok().json(forex_pairs)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: Database = Database::new();
    let data: web::Data<AppState> = web::Data::new(AppState { db: Arc::new(AsyncMutex::new(db)) });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(data.clone())
            .route("/forex/{id}", web::get().to(get_forex_price))
            .route("/forex", web::get().to(get_all_forex_prices))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}