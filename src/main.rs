use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use lettre::{
    message::Mailbox,
    transport::smtp::client::{Certificate, TlsParameters},
    Address, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

async fn root() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct User {
    id: u64,
    name: String,
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    (
        StatusCode::CREATED,
        Json(User {
            id: 1337,
            name: payload.username,
        }),
    )
}

fn create_app(smtp_transport: Option<AsyncSmtpTransport<Tokio1Executor>>) -> Router {
    let router = Router::new().route("/", get(root));
    if let Some(smtp_transport) = smtp_transport {
        router.route(
            "/users",
            post(|req: Json<CreateUser>| async move {
                let sender_address = Address::new("app", "example.localhost").unwrap();
                let sender_mailbox = Mailbox::new(None, sender_address);

                let receiver_address = Address::new("user", "example.localhost").unwrap();
                let receiver_mailbox = Mailbox::new(None, receiver_address);

                let email = Message::builder()
                    .from(sender_mailbox)
                    .to(receiver_mailbox)
                    .subject("Test")
                    .body(format!(
                        "{}さん、ようこそ！以下のリンクをクリックしてメールアドレスを認証してください",
                        req.0.username
                    ))
                    .unwrap();

                smtp_transport.send(email).await.unwrap();

                create_user(req).await
            }),
        )
    } else {
        router.route("/users", post(create_user))
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let selfsigned_certificate = Certificate::from_der(
        include_bytes!("../smtp4dev-data/selfsigned-certificate.cer").to_vec(),
    )
    .unwrap();

    let tls_config = lettre::transport::smtp::client::Tls::Required(
        TlsParameters::builder("localhost".to_owned())
            .add_root_certificate(selfsigned_certificate)
            .dangerous_accept_invalid_certs(true)
            .build()
            .unwrap(),
    );

    let smtp_transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay("0.0.0.0")
        .unwrap()
        .tls(tls_config)
        .port(1025)
        .build::<Tokio1Executor>();

    let app = create_app(Some(smtp_transport));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        let request = Request::builder().uri("/").body(Body::empty()).unwrap();
        let response = create_app(None).oneshot(request).await.unwrap();
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(body, "Hello, world!");
    }
}
