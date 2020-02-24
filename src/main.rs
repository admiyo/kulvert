extern crate clap;
use clap::{Arg, App};


mod access;
mod config;
mod html;
mod identity;
mod links;
mod server;
mod versions;

fn main(){

    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Adam Young <adam@younglogic.com>")
        .about("Delegation Web Service")
        .arg(Arg::with_name("tls")
             .short("t")
             .multiple(false)
             .help("Run with TLS on port 8443"))
        .get_matches();
    
    println!("Matches are: {:?}", matches.args);

    let tls: bool = {
        if let Some(_o) = matches.value_of("tls") {
            true
        }else{
            false
        }
    };
    let _rs = server::server_main(tls);
}
