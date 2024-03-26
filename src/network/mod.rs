// This is the network module

pub mod client; // Declares a submodule

pub fn connect() {
    println!("Connection established from network module.");
    client::connect();
}
