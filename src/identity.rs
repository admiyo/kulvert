use serde::Serialize;


#[derive(Clone, Serialize)]
pub struct Providers {
    providers: Vec<Provider>,

}

#[derive(Clone, Serialize)]
pub struct Provider {
    id: String,
    name: String,
    provider: String,
    enabled: bool,
}


pub fn get_providers() -> Providers{
    let a = Providers {
        providers : [].to_vec()
    };
    return a;
}

