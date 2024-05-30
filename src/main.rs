use clap::{Parser, Subcommand};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Env {
    anthropic_api_key: String,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate {},
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = envy::from_env::<Env>()?;

    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate {} => generate_sentence(&env).await?,
    }

    Ok(())
}

async fn generate_sentence(env: &Env) -> Result<(), reqwest::Error> {
    let resp = reqwest::Client::new()
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &env.anthropic_api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&json!({
            "model": "claude-3-opus-20240229",
            "max_tokens": 1024,
            "messages": [
                {
                    "role": "user",
                    "content": r#"\
                        あなたは中国語の教師です。\
                        たった今私は「几乎」という単語を知りました。\
                        この単語を用いた20文字以内の例文を3つ生成し、文字列の配列を表現するJSONとして出力してください。\
                        JSON以外に言葉は要りません。"#
                }
            ]
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("{:?}", resp);
    Ok(())
}
