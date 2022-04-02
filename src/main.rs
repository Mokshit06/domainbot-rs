use domainbot::{all_domains, is_domain, single_domain};
use dotenv::dotenv;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use std::env;

#[group]
#[default_command(single)]
#[commands(all, single)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv().unwrap();

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
}

fn parse_args<'a>(args: &'a Args) -> Result<&'a str, &'static str> {
    if args.len() > 1 {
        Err("You can only enter one domain at once!")
    } else if let Some(domain) = args.current() {
        Ok(domain)
    } else {
        Err("Please enter a domain!")
    }
}

#[command]
async fn all(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    match parse_args(&args) {
        Ok(domain) => msg.reply(ctx, all_domains(domain).await?).await?,
        Err(reply) => msg.reply(ctx, reply).await?,
    };

    Ok(())
}

#[command]
async fn single(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    match parse_args(&args) {
        Ok(domain) => {
            if !is_domain(domain) {
                msg.reply(ctx, "Please enter a valid domain!").await?
            } else {
                msg.reply(ctx, single_domain(domain).await?).await?
            }
        }
        Err(reply) => msg.reply(ctx, reply).await?,
    };

    Ok(())
}
