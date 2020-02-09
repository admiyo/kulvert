use serde::Serialize;


#[derive(Clone, Serialize)]
pub struct BaseLink {
    hostname: String,
    port: u32
}


#[derive(Clone, Serialize)]
pub struct Link {
    href: String,
    htype: String,
    rel:  String
}

#[derive(Clone, Serialize)]
pub struct MediaType {
    base: String,
    media_type: String
}

#[derive(Clone, Serialize)]
pub struct Version {
    id: String,
    status: String,
    updated: String,
    media_types: Vec<MediaType>,
    links: Vec<Link>
}


#[derive(Clone, Serialize)]
pub struct Versions {
    versions: Vec<Version>
}


fn get_base_url() -> String {
    let base = BaseLink{
        hostname: "localhost".to_string(),
        port:8443
    };
    let base_url =  format!("https://{}:{}", base.hostname, base.port); 
    return  base_url.to_string();
}

fn get_v3_url() -> String {
    format!("{}{}", get_base_url(), "/v3")
}

fn get_json_media_type() -> [MediaType; 1] {
    let a = MediaType {
        base: "application/json".to_string(),
        media_type: "application/vnd.openstack.identity-v3+json".to_string()
    };

    return [a];
}

fn get_v3_summary_links() -> [Link; 1]{
    let a = Link{
        href: get_v3_url().to_string(),
        htype: "text/html".to_string(),
        rel: "self".to_string()
    };
    return [a];
}


fn get_v3_links() -> [Link; 2]{
    let selflink = Link{
        href: get_v3_url().to_string(),
        htype: "text/html".to_string(),
        rel: "self".to_string()
    };


    let userslink = Link{
        href: "https://localhost:8443/v3/idps".to_string(),
        htype: "text/html".to_string(),
        rel: "users".to_string()
    };

    return [selflink, userslink];
}


pub fn get_v3_summary() -> Version{
    let a = Version {
        status: "stable".to_string(),
        updated: "2018-02-28T00:00:00Z".to_string(),
        media_types: get_json_media_type().to_vec(),
        id: "v3.10".to_string(),
        links: get_v3_summary_links().to_vec()
            
    };
    return a;
}

pub fn get_v3() -> Version{
    let a = Version {
        status: "stable".to_string(),
        updated: "2018-02-28T00:00:00Z".to_string(),
        media_types: get_json_media_type().to_vec(),
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
