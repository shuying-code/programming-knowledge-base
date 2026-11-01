use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{CreateTodo, Todo, UpdateTodo},
    state::AppState,
};

/// GET /todos
pub async fn list(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let map = state.read().await;
    let mut todos: Vec<Todo> = map.values().cloned().collect();
    todos.sort_by_key(|t| t.id);
    Json(todos)
}

/// GET /todos/{id}
pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Todo>, AppError> {
    let map = state.read().await;
    map.get(&id).cloned().map(Json).ok_or(AppError::NotFound)
}

/// POST /todos
pub async fn create(
    State(state): State<AppState>,
    Json(input): Json<CreateTodo>,
) -> (StatusCode, Json<Todo>) {
    let todo = Todo {
        id: Uuid::new_v4(),
        title: input.title,
        done: false,
    };
    state.write().await.insert(todo.id, todo.clone());
    (StatusCode::CREATED, Json(todo))
}

/// PUT /todos/{id}
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateTodo>,
) -> Result<Json<Todo>, AppError> {
    let mut map = state.write().await;
    let todo = map.get_mut(&id).ok_or(AppError::NotFound)?;
    if let Some(title) = input.title {
        todo.title = title;
    }
    if let Some(done) = input.done {
        todo.done = done;
    }
    Ok(Json(todo.clone()))
}

/// DELETE /todos/{id}
pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let mut map = state.write().await;
    map.remove(&id)
        .map(|_| StatusCode::NO_CONTENT)
        .ok_or(AppError::NotFound)
}
