use serde::Serialize;


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



fn get_json_media_type() -> [MediaType; 1] {
    let a = MediaType { base: "application/json".to_string(),
                         media_type: "application/vnd.openstack.identity-v3+json".to_string()
    };

    return [a];
}

fn get_v3_links() -> [Link; 1]{
    let a = Link{
        href: "http://localhost:8080/v3/".to_string(),
        htype: "text/html".to_string(),
        rel: "self".to_string()
    };
    return [a];
}

fn get_v3() -> Version{
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
        versions: [get_v3()].to_vec()
    };
    return a;
    
}

/*
{"versions":
 {"values": [{"status": "stable",
              "updated": "2018-02-28T00:00:00Z",
              "media-types": [
                  {"base": "application/json",
                   "type": "application/vnd.openstack.identity-v3+json"}],
              "id": "v3.10", "links": [
                  {"href": "https://kaizen.massopen.cloud:13000/v3/",
                   "rel": "self"}
              ]},
             {"status": "deprecated",
              "updated": "2016-08-04T00:00:00Z",
              "media-types": [
                  {"base": "application/json",
                   "type": "application/vnd.openstack.identity-v2.0+json"}],
              "id": "v2.0",
              "links": [
                  {"href": "https://kaizen.massopen.cloud:13000/v2.0/",
                   "rel": "self"},
                  {"href": "https://docs.openstack.org/",
                   "type": "text/html", "rel": "describedby"}]}
 ]}
}*/
