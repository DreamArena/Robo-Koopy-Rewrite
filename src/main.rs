use std::env;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

#[group]
#[commands(fuzzy, ccz, kurk, railgun)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

        dotenv::dotenv().expect("Failed to load .env file");

   // Initialize the logger to use environment variables.
   //
   // In this case, a good default is setting the environment variable
   // `RUST_LOG` to `debug`.
       tracing_subscriber::fmt::init();

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
       | GatewayIntents::DIRECT_MESSAGES
       | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
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
async fn fuzzy(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "https://media.discordapp.net/attachments/1012752468586799304/1012755508236648528/neppy_and_ratgirl.gif?width=550&height=473").await?;

    Ok(())
}
#[command]
async fn ccz(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "https://cdn.discordapp.com/attachments/1012752468586799304/1012757916769591417/ccz.mov").await?;

    Ok(())
}
#[command]
async fn kurk(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "https://cdn.discordapp.com/attachments/1012752468586799304/1012758318751690823/kurk.jpg").await?;

    Ok(())
}
#[command]
async fn railgun(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Nginx is functioning normally. Maybe. Hopefully. I dunno. Low-effort catch-all page found. Satisfaction not guaranteed. All warranties null and void. All rights, registrations, and trademarks were lost in a matchstick factory fire. PHP calls for re-evaluation of life choices. Python became lost in the dark and accidentally tied itself in a knot. Reality is fiction, and fiction is perception, nothing is real, everything is a solipsistic fever dream. Right is wrong, up is down, heads is tails, dogs and cats living together, mass hysteria...

Yes, I was bored while editing this. =P").await?;

    Ok(())
}
