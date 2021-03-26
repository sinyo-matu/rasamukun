mod upstream;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult, StandardFramework,
};
use serenity::model::channel::Message;

use upstream::{QiitaClient, SOFClient};

use std::env;

#[group]
#[commands(qiita, overflow, kani)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn kani(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(
        ctx,
        r#"
    _~^~^~_
\) /  o o  \ (/
 '-,   -  _'\
  | '----' 
    "#,
    )
    .await?;

    Ok(())
}

#[command]
async fn overflow(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let tag = if args.is_empty() {
        msg.reply(ctx, "なんか指定してくれ").await?;
        return Ok(());
    } else {
        args.single::<String>()?
    };
    let res_body = SOFClient::get_en_questions(&tag).await?;
    let bot_res = match res_body.items.is_empty() {
        true => String::from("見つからなかったサム"),
        _ => {
            let mut bot_res = String::from("以下は関係ありそうサム！\n");
            for item in res_body.items {
                bot_res = format!("{}{}\n", bot_res, item.link);
            }
            bot_res
        }
    };
    msg.reply(ctx, bot_res).await?;

    Ok(())
}

#[command]
async fn qiita(ctx: &Context, msg: &Message) -> CommandResult {
    let res_body = QiitaClient::get_posts().await?;
    let bot_res = match res_body.0.is_empty() {
        true => String::from("見つからなかったサム"),
        _ => {
            let mut bot_res = String::from("Rustの最新記事は以下サム！\n");
            for item in res_body.0 {
                bot_res = format!("{}{}\n", bot_res, item.link);
            }
            bot_res
        }
    };
    msg.reply(ctx, bot_res).await?;

    Ok(())
}
