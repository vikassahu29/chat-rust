use ::models::User;

pub struct Application {
    pub(super) database: Database
}

pub(super) struct Database {
    users: Vec<User>
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
}

pub fn init() -> Application {
    let application = Application {
        database: Database {
            users: vec![User{
                username: String::from("bob"),
                password: String::from("bob")
            }]
        }
    };
    application

}