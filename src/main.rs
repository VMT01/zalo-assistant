use zalo_bot::prelude::{BotAction, BotBuilder};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let bot = BotBuilder::default().build().unwrap();
    let user = bot.get_me().await.unwrap();

    println!("{:?}", user);
}
