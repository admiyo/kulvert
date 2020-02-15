use serde::Serialize;


#[derive(Clone, Serialize)]
pub struct Providers {
    providers: Vec<super::links::Link>,
    links:  Vec<super::links::Link>
}

#[derive(Clone, Serialize)]
pub struct Provider {
    id: String,
    name: String,
    enabled: bool,
    protocols: Vec<Protocol>,
}

#[derive(Clone, Serialize)]
pub struct Protocol {
    id: String,
    name: String,
    mappings: Vec<Mapping>,
}

#[derive(Clone, Serialize)]
pub struct Mapping {
    id: String,
    data: String
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

pub fn get_identity_providers_href() -> String {
    super::versions::get_v3_url() + "/identity_providers"
}

pub fn get_identity_providers_link() -> super::links::Link {
    return super::links::Link{
        href: get_identity_providers_href(),
        htype: "text/html".to_string(),
        rel: "parent".to_string()
    }

}

pub fn get_parent_link() -> super::links::Link {
    return  super::links::Link{
        href: super::versions::get_v3_url().to_string(),
        htype: "text/html".to_string(),
        rel: "parent".to_string()
    }
}

pub fn get_provider_link(provider: &Provider) -> super::links::Link {
    return  super::links::Link{
        href: get_identity_providers_href() + "/" + &provider.id,
        htype: "text/html".to_string(),
        rel: "Provider:".to_string() + &provider.name
    }
}


pub fn get_provider(id: &str) -> Provider{

    //THIS is sample data.
    Provider{
        id: id.to_string(),
        name: "Acme".to_string(),
        enabled: true,
        protocols: [].to_vec(),
    }
}



pub fn get_providers() -> Providers{

    //THIS is sample data.
    let p = Provider{
                id: "ABCD".to_string(),
                name: "Acme".to_string(),
                enabled: true,
                protocols: [].to_vec(),
    };
    let a = Providers {
        providers: [get_provider_link(&p)].to_vec(),
        links: [get_parent_link(),
                 get_identity_providers_link()].to_vec(),

    };
    return a;
}
