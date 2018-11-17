use ::models::*;
use ::application::*;

pub fn login_user(application: &mut Application, user: &User) -> Result<String,&'static str> {
    let db_user = application.database.find_user(&user.username);
    match db_user {
        Some(db_user) => {
            if user.password == db_user.password {
                Ok("token".to_string())
            } else {
                Err("Incorrect Password")
            }
        },
        None => Err("User Not Found")
    }
}

pub fn register_user(application: &mut Application, user: &User) -> Result<(), &'static str>{
    if application.database.add_user(user) {
        Ok(())
    } else {
        Err("User already exists")
    }
}