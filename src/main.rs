mod access;
mod config;
mod html;
mod identity;
mod links;
mod server;
mod versions;

fn main(){
    let insecure: bool = true;
    let _rs = server::server_main(insecure);
}
