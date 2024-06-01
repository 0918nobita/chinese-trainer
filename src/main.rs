use clap::{Parser, Subcommand};
use learning_chinese::{anthropic::generate_sentences, grpc_server};
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Env {
    pub anthropic_api_key: String,
}

static ENV: Lazy<Env> = Lazy::new(|| envy::from_env().unwrap());

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate {
        word: String,
    },
    /// Start a gRPC server
    Serve,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate { word } => {
            let sentences = generate_sentences(&ENV.anthropic_api_key, word).await?;

            println!("{:?}", sentences);
        }
        Commands::Serve => grpc_server::serve(&ENV.anthropic_api_key).await?,
    }

    Ok(())
}
