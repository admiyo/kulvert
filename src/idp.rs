use serde::Serialize;


#[derive(Clone, Serialize)]
pub struct IDPs {
    idps: Vec<IDP>,

}

#[derive(Clone, Serialize)]
pub struct IDP {
    id: String,
    name: String,
    provider: String,
    enabled: bool,
}


pub fn get_idps() -> IDPs{
    let a = IDPs {
        idps : [].to_vec()
    };
    return a;
}

