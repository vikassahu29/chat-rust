extern crate server;
use server::*;

#[test]
fn register_and_login() {
    let mut application = application::init();
    
    let user = models::User {
        username:"asd".to_string(),
        password:"asd".to_string()
    };

    accounts::register_user(&mut application, &user).unwrap();
    let res = accounts::login_user(&mut application, &user);
    match res {
        Ok(_) => println!("Login Successful"),
        _ => panic!("Login Unsuccessful")
    }
}

#[test]
fn register_twice() {
    let mut application = application::init();
    
    let user = models::User {
        username:"asd".to_string(),
        password:"asd".to_string()
    };

    accounts::register_user(&mut application, &user).unwrap();
    if !accounts::register_user(&mut application, &user).is_err() {
        panic!("User registered twice")
    }
}

    