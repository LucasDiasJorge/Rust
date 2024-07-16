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

fn user_update_name(username: String, mut user: User) -> User{
    user.username = username;
    return user;
}

fn user_update_email(email: String, mut user: User) -> User{
    user.email = email;
    return user;
}

fn main(){
    
    let mut user1: User = user_factory("Lucas Dias Jorge".to_string(), "lucas_jorg@hotmail.com".to_string());

    println!("\t Username: {} \n\t Email: {}",user1.username, user1.email);
    
    user1 = user_update_name("Luisa Dias Jorge".to_string(), user1);
    user1 = user_update_email("luisa_dias@hotmail.com".to_string(), user1);
    
    println!("\t Username: {} \n\t Email: {}",user1.username, user1.email);
    
    return ;
}