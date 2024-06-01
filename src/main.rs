use clap::{Parser, Subcommand};
use hello::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};
use learning_chinese::{anthropic::generate_sentences, env::Env};
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
    let env = envy::from_env::<Env>()?;

    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate { word } => generate_sentences(&env, word).await?,
        Commands::Serve => serve().await?,
    }

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
