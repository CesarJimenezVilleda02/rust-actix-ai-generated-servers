use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Article {
    id: u64,
    author: String,
    description: String,
    r#abstract: String,
    platform_link: String,
    download_urls: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    articles: HashMap<u64, Article>,
}

impl Database {
    fn new() -> Self {
        Self {
            articles: HashMap::new(),
        }
    }

    fn insert(&mut self, article: Article) {
        self.articles.insert(article.id, article);
    }

    fn get(&self, id: &u64) -> Option<&Article> {
        self.articles.get(id)
    }

    fn get_all(&self) -> Vec<&Article> {
        self.articles.values().collect()
    }

    fn delete(&mut self, id: &u64) {
        self.articles.remove(id);
    }

    fn update(&mut self, article: Article) {
        self.articles.insert(article.id, article);
    }

    fn save_to_file(&self) -> std::io::Result<()> {
        let data: String = serde_json::to_string(&self)?;
        let mut file: fs::File = fs::File::create("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn load_from_file() -> std::io::Result<Self> {
        let file_content: String = fs::read_to_string("database.json")?;
        let db: Database = serde_json::from_str(&file_content)?;
        Ok(db)
    }
}

struct AppState {
    db: Mutex<Database>,
}

async fn create_article(app_state: web::Data<AppState>, article: web::Json<Article>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.insert(article.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn read_article(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    match db.get(&id.into_inner()) {
        Some(article) => HttpResponse::Ok().json(article),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn read_all_articles(app_state: web::Data<AppState>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    let articles = db.get_all();
    HttpResponse::Ok().json(articles)
}

async fn update_article(app_state: web::Data<AppState>, article: web::Json<Article>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.update(article.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn delete_article(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.delete(&id.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: Database = match Database::load_from_file() {
        Ok(db) => db,
        Err(_) => Database::new(),
    };

    let data: web::Data<AppState> = web::Data::new(AppState { db: Mutex::new(db) });

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
            .route("/article", web::post().to(create_article))
            .route("/article", web::get().to(read_all_articles))
            .route("/article", web::put().to(update_article))
            .route("/article/{id}", web::get().to(read_article))
            .route("/article/{id}", web::delete().to(delete_article))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}