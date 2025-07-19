use std::sync::{Arc, Mutex};

use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder, get, post};
use serde::{Serialize, Deserialize};
use crate::middleware::UserId;
use crate::db::Db;
use crate::routes::utils::TransformJsonKeys;
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize)]
struct CreateTodoResponse {
    message: String
}

#[derive(Serialize, Deserialize)]
struct CreateTodoRequest {
    pub text: Value
}

#[derive(Serialize, Deserialize)]
struct GetTodosResponse {
    todos: Vec<Value>
}

#[post("/todo")]
pub async fn create_todo(user_id: UserId, db: Data<Arc<Mutex<Db>>>, body: Json<CreateTodoRequest>) -> impl Responder {

    let todo = body.text.clone();
    
    let mut db = db.lock().unwrap();
    db.create_todo(user_id.0, todo);
    HttpResponse::Ok().json(CreateTodoResponse {
        message: "Todo created".to_string()
    })
}

#[get("/todos")]
pub async fn get_todos(user_id: UserId, db: Data<Arc<Mutex<Db>>>) -> impl Responder {
    let db = db.lock().unwrap();
    let todos = db.get_todos(user_id.0);

    let transformed: Vec<Value> = todos.into_iter().map(|v| v.transform_keys("TodoApp")).collect();
    HttpResponse::Ok().json(GetTodosResponse {
        todos:transformed
    })
}
