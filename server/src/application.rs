use ::models::User;
use std::collections::HashMap;

pub struct Application {
    pub(super) database: Database
}

pub(super) struct Database {
    users: Vec<User>,
    tokens: HashMap<String, String>
}

impl Database {
    
    pub fn add_user(&mut self, user: &User) -> bool {
        if let None = self.find_user(&user.username){
            self.users.push(user.clone());
            true
        } else {
            false
        }
        
    }

    pub fn find_user(&self, username: &String) -> Option<&User>{
        self.users.iter().find(|ref u| u.username == *username)
    }

    pub fn add_token(&mut self, token: &String, username: &String) {
        self.tokens.insert(token.clone(), username.clone());
    }

    pub fn get_user(&self, token: &String) -> Option<String> {
        match self.tokens.get(token) {
            Some(token) => Some(token.clone()),
            None => None
        }
    }
    
    pub fn remove_token(&mut self, token: &String) {
        self.tokens.remove(token);
    }
}

pub fn init() -> Application {
    let application = Application {
        database: Database {
            users: vec![User{
                username: String::from("bob"),
                password: String::from("bob")
            }],
            tokens: HashMap::new()
        }
    };
    application

}