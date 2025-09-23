use zalo_bot::prelude::types::GetUpdateRequest;
use zalo_bot::prelude::BotBuilder;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let bot = BotBuilder::default().build().unwrap();
    let user = bot
        .get_update(GetUpdateRequest {
            offset: None,
            timeout: Some(60),
            limit: None,
            allowed_updates: None,
        })
        .await
        .unwrap();

    println!("{:?}", user);
}
