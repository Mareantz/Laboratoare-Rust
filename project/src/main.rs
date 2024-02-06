use anyhow::anyhow;
use poise::serenity_prelude::{self as serenity};
use rand::seq::IteratorRandom;
use rand::Rng;
use serde::Deserialize;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::ChannelId;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use std::error::Error;
use std::sync::Arc;
use std::{collections::HashMap, fs, path::Path};
use tokio::time::Duration;
use tracing::error;

#[derive(Deserialize)]
struct Episode {
    title: String,
    runtime: String,
    episode_string: String,
}

struct Bot {
    interval_started: Arc<Mutex<bool>>,
    questions: Vec<(String, String)>,
    current_question: Arc<Mutex<Option<String>>>,
    current_answer: Arc<Mutex<Option<String>>>,
}

impl Bot {
    fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let file_content = fs::read_to_string("src/questions.txt")?;
        let questions: Vec<(String, String)> = file_content
            .lines()
            .map(|line| {
                let (question, answer) = line.split_once(':').unwrap_or((line, ""));
                (question.to_string(), answer.to_string())
            })
            .collect();

        Ok(Self {
            interval_started: Arc::new(Mutex::new(false)),
            questions,
            current_question: Arc::new(Mutex::new(None)),
            current_answer: Arc::new(Mutex::new(None)),
        })
    }
}

async fn handle_points(ctx: &Context, msg: &Message) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/stats.json")?;
    let users_map: HashMap<String, u32> = serde_json::from_str(&input)?;

    if users_map.is_empty() {
        msg.channel_id
            .say(&ctx.http, "No points have been awarded yet!")
            .await?;
        return Ok(());
    }

    let output = serde_json::to_string(&users_map)?;
    fs::write("src/stats.json", output)?;

    let mut leaderboard: Vec<(String, u32)> = Vec::new();
    for (uid, points) in users_map {
        let parsed_uid = uid.parse::<u64>()?;
        let user = ctx.http.get_user(parsed_uid).await?;
        leaderboard.push((user.name, points));
    }

    leaderboard.sort_by(|a, b| b.1.cmp(&a.1));
    let leaderboard_string = leaderboard
        .into_iter()
        .map(|(name, points)| format!("{}: {}", name, points))
        .collect::<Vec<String>>()
        .join("\n");

    msg.channel_id.say(&ctx.http, leaderboard_string).await?;
    Ok(())
}

async fn handle_quote(ctx: &Context, msg: &Message) -> Result<(), Box<dyn Error>> {
    let rng = fs::read_to_string("src/quotes.txt")?;
    let quote_option = rng.lines().choose(&mut rand::thread_rng());

    match quote_option {
        Some(quote) => {
            msg.channel_id.say(&ctx.http, quote).await?;
        }
        None => {
            msg.channel_id.say(&ctx.http, "No quotes found!").await?;
        }
    }

    Ok(())
}

async fn handle_command_with_args(
    ctx: &Context,
    msg: &Message,
    command: &str,
    args: &str,
) -> Result<(), Box<dyn Error>> {
    match command {
        "!doctor" => {
            let path = Path::new("doctors");
            let entries = fs::read_dir(path)?;
            let files: Vec<_> = entries
                .filter_map(Result::ok)
                .map(|res| res.path())
                .collect();
            if let Ok(index) = args.trim().parse::<usize>() {
                if index > 0 && index <= files.len() {
                    let photo = &files[index - 1];
                    msg.channel_id
                        .send_files(&ctx.http, vec![photo], |m| {
                            m.content("Here is your doctor!")
                        })
                        .await?;
                } else {
                    msg.channel_id
                        .say(&ctx.http, "Invalid doctor number!")
                        .await?;
                }
            }
        }
        "!episode" => {
            let input = fs::read_to_string("src/episodes.json")?;
            let episodes: Vec<Episode> = serde_json::from_str(&input)?;
            let mut found = false;
            for episode in episodes {
                if episode.title.to_lowercase().contains(&args.to_lowercase()) {
                    found = true;
                    msg.channel_id
                        .say(
                            &ctx.http,
                            format!(
                                "{} {} {}",
                                episode.title, episode.runtime, episode.episode_string
                            ),
                        )
                        .await?;
                }
            }
            if !found {
                msg.channel_id.say(&ctx.http, "No episode found!").await?;
            }
        }
        _ => {}
    }
    Ok(())
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        match msg.content.as_str() {
            "!quote" => {
                if let Err(e) = handle_quote(&ctx, &msg).await {
                    error!("Error handling quote: {:?}", e);
                }
            }
            "!points" => {
                if let Err(e) = handle_points(&ctx, &msg).await {
                    error!("Error handling points: {:?}", e);
                }
            }
            "!episode" => {
                if let Err(e) = msg
                    .channel_id
                    .say(&ctx.http, "Syntax: !episode [text]")
                    .await
                {
                    error!("Error sending message: {:?}", e);
                }
            }
            "!doctor" => {
                if let Err(e) = msg
                    .channel_id
                    .say(&ctx.http, "Syntax: !doctor [number]")
                    .await
                {
                    error!("Error sending message: {:?}", e);
                }
            }
            _ => (),
        }

        let mut current_question = self.current_question.lock().await;
        let mut current_answer = self.current_answer.lock().await;
        let mut interval_started = self.interval_started.lock().await;
        if let Some(answer) = &*current_answer {
            if msg.content.trim().to_lowercase() == answer.trim().to_lowercase() {
                *current_question = None;
                *current_answer = None;
                *interval_started = true;
                let http = ctx.http.clone();
                let current_question_clone = Arc::clone(&self.current_question);
                let current_answer_clone = Arc::clone(&self.current_answer);
                let questions = self.questions.clone();
                let mut stats: HashMap<u64, u32> = match fs::read_to_string("src/stats.json") {
                    Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
                    Err(_) => HashMap::new(),
                };

                let user_id = *msg.author.id.as_u64();
                let user_score = stats.entry(user_id).or_insert(0);
                *user_score += 1;

                let stats_json_result = serde_json::to_string(&stats);
                match stats_json_result {
                    Ok(stats_json) => {
                        let write_result = fs::write("src/stats.json", stats_json);
                        match write_result {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Error writing file: {:?}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error converting to string: {:?}", e);
                    }
                }

                if let Err(e) = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        "Correct answer! Next question will be posted in 15 seconds!",
                    )
                    .await
                {
                    error!("Error sending message: {:?}", e);
                }

                tokio::spawn(async move {
                    tokio::time::sleep(Duration::from_secs(15)).await;
                    let mut current_question = current_question_clone.lock().await;
                    let mut current_answer = current_answer_clone.lock().await;
                    let random_index = rand::thread_rng().gen_range(0..questions.len());
                    let question_answer_pair = questions[random_index].clone();
                    *current_question = Some(question_answer_pair.0.clone());
                    *current_answer = Some(question_answer_pair.1.clone());

                    if let Err(e) = ChannelId(1194584342354210816)
                        .say(
                            &http,
                            format!("Trivia question: {}", &question_answer_pair.0),
                        )
                        .await
                    {
                        println!("Error sending message: {:?}", e);
                    }
                });
            }
        }

        let (command, args) = msg.content.split_once(' ').unwrap_or((&msg.content, ""));
        if !args.is_empty() {
            if let Err(e) = handle_command_with_args(&ctx, &msg, command, args).await {
                error!("Error handling command: {:?}", e);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let http = ctx.http.clone();
        let current_question_clone = Arc::clone(&self.current_question);
        let current_answer_clone = Arc::clone(&self.current_answer);
        let questions = self.questions.clone();
        let random_index = rand::thread_rng().gen_range(0..questions.len());
        let question_answer_pair = questions[random_index].clone();
        *current_question_clone.lock().await = Some(question_answer_pair.0.clone());
        *current_answer_clone.lock().await = Some(question_answer_pair.1.clone());

        if let Err(e) = ChannelId(1194584342354210816)
            .say(
                &http,
                format!("Trivia question: {}", &question_answer_pair.0),
            )
            .await
        {
            println!("Error sending message: {:?}", e);
        }
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
        .event_handler(Bot::new().unwrap())
        .await
        .expect("Err creating client");

    Ok(client.into())
}
