use domainbot::{all_domains, is_domain, single_domain};
use dotenv::dotenv;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use std::env;
use std::error::Error;

#[group]
#[commands(ping, all, single)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("{:#?}", msg.content);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let discord_token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN doesn't exist");
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("domain "))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(discord_token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }

    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "pong!").await?;

    Ok(())
}

#[command]
async fn all(ctx: &Context, msg: &Message) -> CommandResult {
    let (_, domain) = msg.content.split_once("domain all ").unwrap();
    msg.reply(ctx, all_domains(domain).await?).await?;

    Ok(())
}

#[command]
async fn single(ctx: &Context, msg: &Message) -> CommandResult {
    let (_, domain) = msg.content.split_once("domain single ").unwrap();
    if !is_domain(domain) {
        msg.reply(ctx, "Please enter a valid domain!").await?;
    } else {
        msg.reply(ctx, single_domain(domain).await?).await?;
    }

    Ok(())
}
