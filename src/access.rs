use serde::Serialize;

use uuid::Uuid;


#[derive(Clone, Serialize)]
pub struct Role {
    id: String,
}

#[derive(Clone, Serialize)]
pub struct Namespace {
    id: String,
    roles: Vec<Role>,
    namespaces: Vec<Namespace>,
}


pub fn get_namespace() -> Namespace{
    let a = Namespace {
        id: Uuid::new_v4().to_simple().to_string(),
        roles: [].to_vec(),
        namespaces: [].to_vec(),
    };
    return a;
}

