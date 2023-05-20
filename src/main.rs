#![deny(unused_must_use, unused_imports, rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(missing_docs)]

pub type Aegis<T> = anyhow::Result<T>;

#[tokio::main]
async fn main() -> Aegis<()> {
    std::env::set_var("RUST_LOG", "serenity=debug,sentinel=trace");
    dotenv::dotenv()?;
    env_logger::init();
    println!("Hello, world!");
    Ok(())
}
