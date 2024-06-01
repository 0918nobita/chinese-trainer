use super::anthropic::generate_sentences;
use tonic::{transport::Server, Request, Response, Status};

mod sentence {
    tonic::include_proto!("sentence");
}

use sentence::{
    sentence_service_server::{SentenceService, SentenceServiceServer},
    GenerateSentenceReply, GenerateSentenceRequest, Sentence,
};

#[derive(Debug)]
struct MySentenceService {
    pub anthropic_api_key: String,
}

#[tonic::async_trait]
impl SentenceService for MySentenceService {
    async fn generate(
        &self,
        request: Request<GenerateSentenceRequest>,
    ) -> Result<Response<GenerateSentenceReply>, Status> {
        println!("Got a request: {:?}", request);

        let Ok(sentences) =
            generate_sentences(&self.anthropic_api_key, &request.into_inner().word).await
        else {
            return Err(Status::internal("Failed to generate sentences"));
        };

        let reply = GenerateSentenceReply {
            sentences: sentences
                .into_iter()
                .map(|sentence| Sentence {
                    zh: sentence.zh,
                    ja: sentence.ja,
                })
                .collect(),
        };

        Ok(Response::new(reply))
    }
}

pub async fn serve(anthropic_api_key: &str) -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse()?;
    let sentence_service = MySentenceService {
        anthropic_api_key: anthropic_api_key.to_owned(),
    };

    Server::builder()
        .add_service(SentenceServiceServer::new(sentence_service))
        .serve(addr)
        .await?;

    Ok(())
}
