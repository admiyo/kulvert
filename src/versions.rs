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
    "/v3".to_string()
}

pub fn build_v3_url(link: &str) -> String {
    format!("/v3/{}", link).to_string()
}

pub fn get_v3_link(rel: &str) -> super::links::Link {
    return  super::links::Link{
        href: get_v3_url(),
        htype: "text/html".to_string(),
        rel: rel.to_string()
    }
}

fn get_v3_summary_links() -> [super::links::Link; 1]{
    let a = super::links::Link{
        href: get_v3_url(),
        htype: "text/html".to_string(),
        rel: "v3".to_string()
    };
    return [a];
}


fn get_v3_links() -> [super::links::Link; 3]{
    let selflink = super::links::Link{
        href: get_v3_url(),
        htype: "text/html".to_string(),
        rel: "self".to_string()
    };


    let identity_providers_link = super::links::Link{
        href: build_v3_url("identity_providers"),
        htype: "text/html".to_string(),
        rel: "identity_providers".to_string()
    };

    let namespaces_link = super::links::Link{
        href: build_v3_url("namespaces"),
        htype: "text/html".to_string(),
        rel: "namespaces".to_string()
    };


    return [selflink, identity_providers_link, namespaces_link];
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
