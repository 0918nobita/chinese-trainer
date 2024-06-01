use serde::Deserialize;

#[derive(Deserialize)]
pub struct Env {
    pub anthropic_api_key: String,
}
