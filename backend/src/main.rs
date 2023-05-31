use tonic::{transport::Server, Request, Response, Status};
use word::{
    word_service_server::{WordService, WordServiceServer},
    WordRequest, WordResponse,
};

pub mod word {
    tonic::include_proto!("word");
}

struct MyWordService;

#[tonic::async_trait]
impl WordService for MyWordService {
    async fn get_word(
        &self,
        request: Request<WordRequest>,
    ) -> Result<Response<WordResponse>, Status> {
        let id = request.into_inner().id;

        tracing::info!("get_word(id: {})", id);

        let word = WordResponse {
            id,
            simplified_chinese_characters: "你好".to_owned(),
            pinyin: "nĭ hăo".to_owned(),
            meaning: "こんにちは".to_owned(),
        };

        Ok(Response::new(word))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr: std::net::SocketAddr = "[::1]:50051".parse()?;
    let word_service = MyWordService;

    Server::builder()
        .add_service(WordServiceServer::new(word_service))
        .serve(addr)
        .await?;

    Ok(())
}
