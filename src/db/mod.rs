use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub role: UserRole,
    pub username: String,
    pub password: String,
    pub todos: Vec<Value>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    User,
}

#[derive(Clone, Debug)]
pub struct Db {
    pub index: u32,
    pub users: Vec<User>,
}

impl Default for Db {
    fn default() -> Self {
        Db {
            index: 0,
            users: vec![],
        }
    }
}

impl Db {
    pub fn create_user(&mut self, username: String, password: String, role: UserRole) -> String {
        self.users.push(User {
            id: self.index.to_string(),
            username,
            password,
            todos: vec![],
            role,
        });
        self.index = self.index + 1;
        (self.index - 1).to_string()
    }

    pub fn get_user_by_username(&mut self, username: String) -> Option<&User> {
        println!("users: {:?}", self.users);
        println!("username: {:?}", username);
        self.users.iter().find(|u| {
            u.username == username
        })
    }

    pub fn create_todo(&mut self, user_id: String, todo: Value)  {
        println!("users: {:?}", self.users);
        let user = self.users.iter_mut().find(|u| {
            u.id == user_id
        });

        println!("user: {:?}", user);
        println!("user_id: {:?}", user_id);
        println!("todo: {:?}", todo);

        match user {
            Some(u) => {
                u.todos.push(todo.clone())
            },
            None => {
                panic!("User not found");
            }
        };
    }

    pub fn get_todos(&self, user_id: String) -> Vec<Value> {
        let user = self.users.iter().find(|u| {
            u.id == user_id
        });

        match user {
            Some(u) => {
                u.todos.clone()
            },
            None => {
                vec![]
            }
        }
    }
}