mod server_config;
mod sandbox;

use crate::sandbox::Sandbox;
use crate::server_config::Settings;

const SOFTWARE_NAME_SIMPLE: &str = "accela";
const SOFTWARE_NAME: &str = "ACCELA";

fn check_os() {
    use std::env::consts::OS;
    if OS != "linux" {
        panic!("{} only supports Linux. Detected OS: '{}'", SOFTWARE_NAME_SIMPLE, OS);
    }
}

fn main() {
    check_os();

    println!("\nBOOTING {} ///\n", SOFTWARE_NAME);

    println!("Loading settings ...");
    let settings = Settings::new();
    println!(" node: \"{}\"", settings.server.name);
    println!(" boot: {:?}", settings.server.boot);

    println!("\nCreating a new container environment ...");
    let sbox: Sandbox = Sandbox::new();
    println!("\nBooting telnet module ...");
    sbox.run("accela-telnet");

    println!()
}
