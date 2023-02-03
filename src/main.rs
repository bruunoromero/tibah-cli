mod client;
mod config;
mod models;

use clap::{Parser, Subcommand};
use client::Client;
use config::Config;
use tokio;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Habits {},
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let config = Config::from_env();
    let client = Client::new(&config.api_key, "https://tibah.vercel.app");

    let habits = client.get_habits().await?;

    println!("{:#?}", habits);

    Ok(())
}
