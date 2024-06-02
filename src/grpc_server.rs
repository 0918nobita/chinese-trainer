use crate::{
    anthropic::generate_sentences,
    todo_db::{StatusVO, TodoEntity, TodoRepository, TodoRepositoryForMemory},
};
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
    AllTodoReply, AllTodoRequest, CreateTodoReply, CreateTodoRequest, DeleteTodoReply,
    DeleteTodoRequest, FindTodoReply, FindTodoRequest, TodoItem,
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

fn todo_entity_to_proto(todo: &TodoEntity) -> TodoItem {
    TodoItem {
        id: todo.id,
        text: todo.text.clone(),
        status: match todo.status() {
            StatusVO::Todo => 0,
            StatusVO::Done => 1,
        },
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
    async fn all(
        &self,
        _request: Request<AllTodoRequest>,
    ) -> Result<Response<AllTodoReply>, Status> {
        let todo_list = self.todo_repository.all();

        Ok(Response::new(AllTodoReply {
            items: todo_list.iter().map(todo_entity_to_proto).collect(),
        }))
    }

    async fn find(
        &self,
        request: Request<FindTodoRequest>,
    ) -> Result<Response<FindTodoReply>, Status> {
        let Some(todo) = self.todo_repository.find(request.into_inner().id) else {
            return Err(Status::not_found("The specified todo was not found"));
        };
        Ok(Response::new(FindTodoReply {
            todo: Some(todo_entity_to_proto(&todo)),
        }))
    }

    async fn create(
        &self,
        request: Request<CreateTodoRequest>,
    ) -> Result<Response<CreateTodoReply>, Status> {
        Ok(Response::new(CreateTodoReply {
            todo: Some(todo_entity_to_proto(
                &self.todo_repository.create(request.into_inner().text),
            )),
        }))
    }

    async fn delete(
        &self,
        request: Request<DeleteTodoRequest>,
    ) -> Result<Response<DeleteTodoReply>, Status> {
        let Some(_) = self.todo_repository.delete(request.into_inner().id) else {
            return Err(Status::not_found("The specified todo was not found"));
        };
        Ok(Response::new(DeleteTodoReply {}))
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
