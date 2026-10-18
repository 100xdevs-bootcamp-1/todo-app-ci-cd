use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub done: bool,
}

#[derive(Deserialize)]
pub struct CreateTodo {
    title: String,
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    title: Option<String>,
    done: Option<bool>,
}

#[derive(Default)]
struct Store {
    todos: Vec<Todo>,
    next_id: u64,
}

type AppState = Arc<Mutex<Store>>;

pub fn app() -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/{id}", patch(update_todo).delete(delete_todo))
        .with_state(AppState::default())
}

async fn list_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    Json(state.lock().unwrap().todos.clone())
}

async fn create_todo(
    State(state): State<AppState>,
    Json(input): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), StatusCode> {
    let title = input.title.trim();
    if title.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut store = state.lock().unwrap();
    store.next_id += 1;
    let todo = Todo {
        id: store.next_id,
        title: title.to_string(),
        done: false,
    };
    store.todos.push(todo.clone());
    Ok((StatusCode::CREATED, Json(todo)))
}

async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(input): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let mut store = state.lock().unwrap();
    let todo = store
        .todos
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;
    if let Some(title) = input.title {
        todo.title = title;
    }
    if let Some(done) = input.done {
        todo.done = done;
    }
    Ok(Json(todo.clone()))
}

async fn delete_todo(State(state): State<AppState>, Path(id): Path<u64>) -> StatusCode {
    let mut store = state.lock().unwrap();
    let before = store.todos.len();
    store.todos.retain(|t| t.id != id);
    if store.todos.len() < before {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
