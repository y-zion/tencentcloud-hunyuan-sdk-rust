use anyhow::Result;
use tencentcloud_hunyuan_sdk::models::{ChatCompletionsRequest, Message};
use tencentcloud_hunyuan_sdk::{Client, ClientBuilder, Credential, Region};

#[tokio::main]
async fn main() -> Result<()> {
    let secret_id = std::env::var("TENCENTCLOUD_SECRET_ID")?;
    let secret_key = std::env::var("TENCENTCLOUD_SECRET_KEY")?;

    let client: Client = ClientBuilder::new()
        .credential(Credential {
            secret_id,
            secret_key,
            token: None,
        })
        .region(Region::ApGuangzhou)
        .build();

    let req = ChatCompletionsRequest {
        model: Some("hunyuan-lite".to_string()),
        messages: vec![Message {
            role: "user".into(),
            content: "Hello, Hunyuan!".into(),
        }],
        temperature: Some(0.7),
        top_p: Some(0.95),
        stream: Some(false),
    };

    let resp = client.chat_completions(&req).await?;
    println!("{:?}", resp);

    Ok(())
}
