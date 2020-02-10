use serde::Serialize;


#[derive(Clone, Serialize)]
pub struct Providers {
    providers: Vec<Provider>,
    protocols: Vec<Protocol>,
    mappings: Vec<Mapping>,
    accounts: Vec<Account>
}

#[derive(Clone, Serialize)]
pub struct Provider {
    id: String,
    name: String,
    provider: String,
    enabled: bool,
}

#[derive(Clone, Serialize)]
pub struct Protocol {
    id: String,
    name: String,
    provider: String,
    enabled: bool,
}

#[derive(Clone, Serialize)]
pub struct Mapping {
    id: String,
    name: String,
    provider: String,
    enabled: bool,
}

#[derive(Clone, Serialize)]
pub struct Account {
    id: String,
    name: String,
    provider: String,
    enabled: bool,
}

#[derive(Clone, Serialize)]
pub struct Login {
    id: String,
    name: String,
    provider: String,
    enabled: bool,
}


pub fn get_providers() -> Providers{
    let a = Providers {
        providers : [].to_vec(),
        protocols: [].to_vec(),
        mappings: [].to_vec(),
        accounts: [].to_vec(),
    };
    return a;
}

