use anyhow::Context;
use indoc::formatdoc;
use serde::{Deserialize, Serialize};
use serde_json::json;

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

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneratedSentence {
    pub ja: String,
    pub zh: String,
}

pub async fn generate_sentences(
    anthropic_api_key: &str,
    word: &str,
) -> anyhow::Result<Vec<GeneratedSentence>> {
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
        .header("x-api-key", anthropic_api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&request_body)
        .send()
        .await?
        .json::<MessageResponse>()
        .await
        .with_context(|| "Failed to deserialize HTTP response")?;

    let MessageContent::Text { text } = resp.content.first().with_context(|| "empty response")?;

    serde_json::from_str::<Vec<GeneratedSentence>>(&format!("[{}", &text))
        .with_context(|| "Failed to parse message as JSON")
}
