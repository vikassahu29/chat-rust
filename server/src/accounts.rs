use ::models::*;
use ::application::*;
use uuid::Uuid;

pub fn login_user(application: &mut Application, user: &User) -> Result<String,&'static str> {
    let ref mut db = application.database;
    let mut token = String::new();
    let result = 
    {
        match db.find_user(&user.username) {
            Some(db_user) => {
                if user.password == db_user.password {
                    token = Uuid::new_v4().to_string();
                    Ok(token.clone())
                } else {
                    Err("Incorrect Password")
                }
            },
            None => Err("User Not Found")
        }
    };
    if let Ok(_) = result {
        db.add_token(&token, &user.username);
    }
    result
    
    
}

pub fn register_user(application: &mut Application, user: &User) -> Result<(), &'static str>{
    if application.database.add_user(user) {
        Ok(())
    } else {
        Err("User already exists")
    }
}

pub fn get_user(application: &mut Application, token: &String) -> Option<String> {
    application.database.get_user(token)
}

pub fn logout_user(application: &mut Application, token: &String) {
    application.database.remove_token(token);
}