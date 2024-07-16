struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn user_factory(username: String, email: String) -> User{
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn main() {
    
    let user1: User = user_factory("Lucas Dias Jorge".to_string(), "lucas_jorg@hotmail.com".to_string());

    println!("\t Username: {} \n\t Email: {}",user1.username, user1.email);
    return;
}