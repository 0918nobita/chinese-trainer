use anyhow::Context;
use clap::{Parser, Subcommand};
use hello::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};
use indoc::formatdoc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tonic::{transport::Server, Request, Response, Status};

pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello, {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum MessageContent {
    Text { text: String },
}

#[derive(Debug, Deserialize)]
struct MessageResponse {
    content: Vec<MessageContent>,
    #[allow(dead_code)]
    id: String,
}

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
    Generate {
        word: String,
    },
    /// Start a gRPC server
    Serve,
}

#[derive(Debug, Deserialize, Serialize)]
struct GeneratedSentence {
    ja: String,
    zh: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = envy::from_env::<Env>()?;

    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate { word } => generate_sentences(&env, word).await?,
        Commands::Serve => serve().await?,
    }

    Ok(())
}

async fn generate_sentences(env: &Env, word: &str) -> anyhow::Result<()> {
    let json_schema = json!({
        "type": "array",
        "items": {
            "type": "object",
            "properties": {
                "ja": {
                    "type": "string",
                },
                "zh": {
                    "type": "string",
                }
            },
            "required": ["ja", "zh"],
            "additionalProperties": false,
        },
    });

    let json_schema = serde_json::to_string_pretty(&json_schema).unwrap();

    let prompt = formatdoc! {"
        あなたは中国語の教師です。
        たった今私は「{}」という単語を知りました。
        この単語を用いた20文字以内の例文を3つ生成し、バッククォート無しで以下のJSONスキーマに沿ったJSONだけを出力してください。
        ```json
        {}
        ```",
        word,
        json_schema
    };

    let request_body = json!({
        "model": "claude-3-opus-20240229",
        "max_tokens": 1024,
        "messages": [
            {
                "role": "user",
                "content": prompt
            },
            {
                "role": "assistant",
                "content": "[",
            },
        ],
    });

    let resp = reqwest::Client::new()
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &env.anthropic_api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&request_body)
        .send()
        .await?
        .json::<MessageResponse>()
        .await
        .with_context(|| "Failed to deserialize HTTP response")?;

    let MessageContent::Text { text } = resp.content.first().with_context(|| "empty response")?;

    let sentences = serde_json::from_str::<Vec<GeneratedSentence>>(&format!("[{}", &text))
        .with_context(|| "Failed to parse message as JSON")?;

    println!("{:?}", sentences);

    Ok(())
}

async fn serve() -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
