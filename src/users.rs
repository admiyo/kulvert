use serde::Serialize;


#[derive(Clone, Serialize)]
pub struct Users {
    users: Vec<Users>,

}

#[derive(Clone, Serialize)]
pub struct User {
    id: String,
    name: String,
    provider: String,
    enabled: bool,
}


pub fn get_users() -> Users{
    let a = Users {
        users : [].to_vec()
    };
    return a;
}

