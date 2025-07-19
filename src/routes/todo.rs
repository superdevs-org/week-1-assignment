use std::sync::{Arc, Mutex};

use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder, get, post};
use serde::{Serialize, Deserialize};
use crate::middleware::UserId;
use crate::db::Db;
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

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => "".to_string(),
            }
        })
        .collect::<String>()
}

#[post("/todo")]
pub async fn create_todo(user_id: UserId, db: Data<Arc<Mutex<Db>>>, body: Json<CreateTodoRequest>) -> impl Responder {
    let input = body.text.clone();

    let mut transformed = Map::new();

    if let Some(obj) = input.as_object() {
        for (key, value) in obj {
            let pascal_key = to_pascal_case(key);

            let new_key = format!("TodoApp{}", pascal_key);
            transformed.insert(new_key, value.clone());
        }
    }

    let todo = Value::Object(transformed);
    
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
    HttpResponse::Ok().json(GetTodosResponse {
        todos
    })
}
