use serde::Serialize;


#[derive(Clone, Serialize)]
pub struct Version {
    id: String,
    status: String,
    updated: String,
    media_types: Vec<super::links::MediaType>,
    links: Vec<super::links::Link>
}


#[derive(Clone, Serialize)]
pub struct Versions {
    versions: Vec<Version>
}



pub fn get_v3_url() -> String {
    format!("{}{}", super::links::get_base_url(), "/v3")
}


fn get_v3_summary_links() -> [super::links::Link; 1]{
    let a = super::links::Link{
        href: get_v3_url().to_string(),
        htype: "text/html".to_string(),
        rel: "self".to_string()
    };
    return [a];
}


fn get_v3_links() -> [super::links::Link; 3]{
    let selflink = super::links::Link{
        href: get_v3_url().to_string(),
        htype: "text/html".to_string(),
        rel: "self".to_string()
    };


    let identity_providers_link = super::links::Link{
        href: "https://localhost:8443/v3/identity_providers".to_string(),
        htype: "text/html".to_string(),
        rel: "identity_providers".to_string()
    };

    let namespace_link = super::links::Link{
        href: "https://localhost:8443/v3/namespace".to_string(),
        htype: "text/html".to_string(),
        rel: "namespace".to_string()
    };

    
    return [selflink, identity_providers_link, namespace_link];
}


pub fn get_v3_summary() -> Version{
    let a = Version {
        status: "stable".to_string(),
        updated: "2018-02-28T00:00:00Z".to_string(),
        media_types: super::links::get_json_media_type().to_vec(),
        id: "v3.10".to_string(),
        links: get_v3_summary_links().to_vec()
            
    };
    return a;
}

pub fn get_v3() -> Version{
    let a = Version {
        status: "stable".to_string(),
        updated: "2018-02-28T00:00:00Z".to_string(),
        media_types: super::links::get_json_media_type().to_vec(),
        id: "v3.10".to_string(),
        links: get_v3_links().to_vec()
            
    };

    return a;
}

pub fn get_versions() -> Versions{
    let a = Versions{
        versions: [get_v3_summary()].to_vec()
    };
    return a;
    
}
