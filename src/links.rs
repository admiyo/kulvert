use serde::Serialize;


#[derive(Clone, Serialize)]
pub struct BaseLink {
    hostname: String,
    port: u32
}


#[derive(Clone, Serialize)]
pub struct Link {
    pub href: String,
    pub htype: String,
    pub rel:  String
}

#[derive(Clone, Serialize)]
pub struct MediaType {
    pub base: String,
    pub media_type: String
}

pub fn get_json_media_type() -> [MediaType; 1] {
    let a = MediaType {
        base: "application/json".to_string(),
        media_type: "application/vnd.openstack.identity-v3+json".to_string()
    };

    return [a];
}
