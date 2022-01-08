use std::{env, process};
mod models;
use models::config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let user_input = config::Config::new(args).unwrap_or_else(|err| {
        println!("Erreur survenue: {}", err);
        process::exit(1);
    });
    println!("{:?}", user_input);
}
