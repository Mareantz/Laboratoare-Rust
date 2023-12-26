use anyhow::anyhow;
use rand::seq::IteratorRandom;
use serde::Deserialize;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use std::{fs, path::Path};
use tracing::{error, info};

#[derive(Deserialize)]
struct Episode {
    title: String,
    runtime: String,
    episode: String,
}

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!doctor" {
            if let Err(e) = msg
                .channel_id
                .say(&ctx.http, "Command usage: !doctor [number]")
                .await
            {
                error!("Error sending message: {:?}", e);
            }
        }
        if msg.content == "!quote" {
            let rng = fs::read_to_string("src/quotes.txt");
            match rng {
                Ok(rng) => {
                    let quote = rng.lines().choose(&mut rand::thread_rng()).unwrap();
                    if let Err(e) = msg.channel_id.say(&ctx.http, quote).await {
                        error!("Error sending message: {:?}", e);
                    }
                }
                Err(e) => error!("Error reading file: {:?}", e),
            }
        }
        let content = msg.content.split_once(' ');
        if let Some((command, args)) = content {
            if command == "!doctor" {
                let path = Path::new("doctors");
                let entries = fs::read_dir(path).expect("Unable to list files in the directory");
                let files: Vec<_> = entries
                    .filter_map(Result::ok)
                    .map(|res| res.path())
                    .collect();
                if let Ok(index) = args.trim().parse::<usize>() {
                    if index > 0 && index <= files.len() {
                        let photo = &files[index - 1];
                        if let Err(e) = msg
                            .channel_id
                            .send_files(&ctx.http, vec![photo], |m| {
                                m.content("Here is your photo!")
                            })
                            .await
                        {
                            error!("Error sending photo: {:?}", e);
                        }
                    } else if let Err(e) =
                        msg.channel_id.say(&ctx.http, "Invalid photo number!").await
                    {
                        error!("Error sending message: {:?}", e);
                    }
                } else if let Err(e) = msg.channel_id.say(&ctx.http, "Invalid number!").await {
                    error!("Error sending message: {:?}", e);
                }
            }
            if command == "!episode" {
                let input = fs::read_to_string("src/episodes.json");
                let episodes: Vec<Episode> = serde_json::from_str(&input.unwrap()).unwrap();
                for episode in episodes {
                    if episode.title.to_lowercase().contains(&args.to_lowercase()) {
                        if let Err(e) = msg
                            .channel_id
                            .say(
                                &ctx.http,
                                format!(
                                    "{} {} {}",
                                    episode.title, episode.runtime, episode.episode
                                ),
                            )
                            .await
                        {
                            error!("Error sending message: {:?}", e);
                        }
                    }
                }
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
