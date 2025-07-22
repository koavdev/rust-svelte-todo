use axum::{
    routing::{get},
    Router, Json, Form,
    extract::{State, Path}
};
use std::net::SocketAddr;
use serde::{Serialize, Deserialize};
use axum_error::Result;
use sqlx::sqlite::SqlitePool;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<()> {
    // Get enviornment variables
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;

    // Create router for server
    let app = Router::new()
        .route("/", get(list))
        .route("/create", get(create))
        .route("/delete/:id", get(delete))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    // Start server!
    let address = SocketAddr::from(([0,0,0,0], 8000));
    Ok(axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}

#[derive(Serialize, Deserialize)]
struct Todo {
    id: i64,
    description: String,
    done: bool
}

async fn list(State(pool): State<SqlitePool>) -> Result<Json<Vec<Todo>>> {
    // List all todos
    let todos = sqlx::query_as!(Todo, "SELECT id, description, done FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await?;
    Ok(Json(todos))
}

async fn create(State(pool): State<SqlitePool>, Form(todo): Form<Todo>) -> Result<String> {
    // Create todo
    sqlx::query!("INSERT INTO todos (description) VALUES (?)", todo.description).execute(&pool).await?;
    Ok(format!("Successfully inserted todo!"))
}

async fn delete(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<String> {
    // Delete todo
    sqlx::query!("DELETE FROM todos where id = ?", id).execute(&pool).await?;
    Ok(format!("Successfully deleted todo!"))
}