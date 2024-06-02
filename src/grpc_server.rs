use crate::todo_db::{self, TodoRepository, TodoRepositoryForMemory};

use super::anthropic::generate_sentences;
use tonic::{transport::Server, Request, Response, Status};

mod sentence {
    tonic::include_proto!("sentence");
}

mod todo {
    tonic::include_proto!("todo");
}

use sentence::{
    sentence_service_server::{SentenceService, SentenceServiceServer},
    GenerateSentenceReply, GenerateSentenceRequest, Sentence,
};
use todo::{
    todo_service_server::{TodoService, TodoServiceServer},
    GetTodoListReply, GetTodoListRequest, TodoItem,
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

#[derive(Debug)]
struct MyTodoService<R>
where
    R: TodoRepository,
{
    todo_repository: R,
}

impl<R> MyTodoService<R>
where
    R: TodoRepository,
{
    pub fn new(todo_repository: R) -> Self {
        Self { todo_repository }
    }
}

#[tonic::async_trait]
impl<R> TodoService for MyTodoService<R>
where
    R: TodoRepository,
{
    async fn get_todo_list(
        &self,
        _request: Request<GetTodoListRequest>,
    ) -> Result<Response<GetTodoListReply>, Status> {
        let todo_list = self.todo_repository.get_todo_list();

        Ok(Response::new(GetTodoListReply {
            items: todo_list
                .iter()
                .map(|todo| TodoItem {
                    id: todo.id,
                    text: todo.text.clone(),
                    status: match todo.status() {
                        todo_db::Status::Todo => 0,
                        todo_db::Status::Done => 1,
                    },
                })
                .collect(),
        }))
    }
}

pub async fn serve(anthropic_api_key: &str) -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse()?;
    let sentence_service = MySentenceService {
        anthropic_api_key: anthropic_api_key.to_owned(),
    };
    let todo_service = MyTodoService::new(TodoRepositoryForMemory::default());

    Server::builder()
        .add_service(SentenceServiceServer::new(sentence_service))
        .add_service(TodoServiceServer::new(todo_service))
        .serve(addr)
        .await?;

    Ok(())
}
