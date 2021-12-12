use serenity::{
    async_trait,
    client::bridge::gateway::GatewayIntents,
    framework::standard::{
        Args, CommandGroup, HelpOptions, help_commands::*,
        macros::{command, group, help},
        CommandResult, StandardFramework, WithWhiteSpace,
    },
    model::{
        channel::Message,
        id::{EmojiId, UserId},
        prelude::{ChannelId, GuildId, Member, Mention, Reaction, ReactionType},
    },
    prelude::*,
};

use std::{collections::{hash_map::RandomState, HashSet}, env, fmt::Debug, sync::Arc};

use regex::Regex;

use dashmap::DashMap;

fn err_say<A, B: Debug>(x: Result<A, B>) {
    if let Err(why) = x {
        println!("Error: {:?}", why);
    }
}

const JOIN_MESSAGE: &str =
  "";
const ASK_ATTACHMENT: &str = "";
const NEEDS_SINGLE_ATTACHMENT: &str =
    "";
const BOTS_OWN_SNOWFLAKE: UserId = UserId();
const VERIFICATION_CHANNEL: ChannelId = ChannelId();
const INITIAL_MESSAGE_ASKS_REACTION: &str =
    "";
const FULL_NAME_PLEASE: &str = "";
const JUST_FINISHED_NON_STUDENT_FORM: &str = "";
const JUST_FINISHED_STUDENT_FORM : &str = "";
const COMPLETED_STUDENT_FORM : &str = "";
const COMPLETED_NON_STUDENT_FORM : &str = "";
const INFO_ABOUT_BOT : &str = "";
const EASTER_EGG : &str = "7B C 0 E 7D 5E 1B 57 19 72 4 5C 14 5C 4 7E E 7A F 46 3 4 77 41 77 73 5 3 3 7C 5 39 1 0 7F 36 35 3F 77 73 76 75 68";
const PRIVACY_POLICY : &str = "";

#[group]
#[commands(info, privacy, easter_egg)]
struct General;

enum Form {
    UnnamedStudent,
    NamedStudent(String),
    CompletedStudent,
    CompletedNonStudent,
}

struct Forms;

impl TypeMapKey for Forms {
    type Value = Arc<DashMap<UserId, Form, RandomState>>;
}

#[command]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    err_say(msg.author.direct_message(ctx, |m| m.content(INFO_ABOUT_BOT)).await);
    Ok(())
}
#[command]
async fn easter_egg(ctx: &Context, msg: &Message) -> CommandResult {
    err_say(msg.author.direct_message(ctx, |m| m.content(EASTER_EGG)).await);
    Ok(())
}

#[command]
async fn privacy(ctx: &Context, msg: &Message) -> CommandResult {
    err_say(msg.author.direct_message(ctx, |m| m.content(PRIVACY_POLICY)).await);
    Ok(())
}

async fn message_not_self(ctx: Context, msg: Message) {
    let forms: Arc<DashMap<UserId, Form, RandomState>> = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<Forms>()
            .expect("Expected Form in TypeMap.")
            .clone()
    };

    let entry = forms.get(&msg.author.id);
                    let student_emoji = ReactionType::Unicode("📚".to_string());
                    let alumni_emoji = ReactionType::Unicode("🎓".to_string());
                    let imposter_emoji = ReactionType::Unicode("👽".to_string());
    match entry {
        None => {
            drop(entry);
            err_say(
                msg.author
                    .direct_message(&ctx, |m| m.content(INITIAL_MESSAGE_ASKS_REACTION).reactions(vec![student_emoji, alumni_emoji, imposter_emoji]))
                    .await
            )
        }

        Some(form_ref) => match &*form_ref {
            Form::UnnamedStudent => {

                drop(form_ref);
                let check_full_name =
                    Regex::new(r"^(\p{Z})*(\p{L})+(((\p{Z})+\p{L}+)|(\p{Z})+_+)+(\p{Z})*$")
                        .unwrap();
                if check_full_name.is_match(&msg.content) {
                    // hello OCaml my old friend
                    forms.insert(msg.author.id, Form::NamedStudent(msg.content));
                    err_say(
                        msg.author
                            .direct_message(&ctx, |m| m.content(ASK_ATTACHMENT)).await);
                } else {
                                        err_say(
                        msg.author
                            .direct_message(&ctx, |m| m.content(FULL_NAME_PLEASE))
                            .await)
                }
            }

            Form::CompletedStudent => {
                drop(form_ref);
                err_say(
                    msg.author
                        .direct_message(&ctx, |m| m.content(COMPLETED_STUDENT_FORM))
                        .await,
                )
            }
            Form::CompletedNonStudent => {
                drop(form_ref);
                err_say(
                    msg.author
                        .direct_message(&ctx, |m| m.content(COMPLETED_NON_STUDENT_FORM))
                        .await,
                )
            }
            Form::NamedStudent(name) => match &msg.attachments[..] {
                [i] => {
                    println!("got attachemnt");
                    let useful_info = format!(
                        "Discord ID: {}\nName: {}\nAttachment: {}",
                        Mention::from(msg.author.id),
                        name,
                        i.proxy_url
                    );
                    drop(form_ref);
                    forms.insert(msg.author.id, Form::CompletedStudent);
                    err_say(VERIFICATION_CHANNEL.say(&ctx, useful_info).await);
                    err_say(
                        msg.author.direct_message(&ctx, |m| m.content(JUST_FINISHED_STUDENT_FORM)).await);
                }
                [] | _ => {
                    drop(form_ref);
                    err_say(
                        msg.author
                            .direct_message(&ctx, |m| m.content(NEEDS_SINGLE_ATTACHMENT))
                            .await,
                    );
                }
            },
        },
    }
}

// this may use .await wrong at times
struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, _: GuildId, new_member: Member) {
        // optional errror message 'Error sending a new member a message'
        //let emojis : Vec<EmojiId> = vec![STUDENT_EMOJI, ALUMNI_EMOJI, IMPOSTER_EMOJI];
        //

        let student_emoji = ReactionType::Unicode("📚".to_string());
        let alumni_emoji = ReactionType::Unicode("🎓".to_string());
        let imposter_emoji = ReactionType::Unicode("👽".to_string());
        err_say(
            new_member
                .user
                .direct_message(&ctx, |m| {
                    m.content(JOIN_MESSAGE).reactions(vec![
                        student_emoji,
                        alumni_emoji,
                        imposter_emoji,
                    ])
                })
                .await,
        )
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        // intentional bug because it reduces unnecessary complexity
        // don't check the person is reacting to one of the bot's messages
        // just checking it is in a direct message
        if let Some(_) = reaction.guild_id {
            return ();
        }

        if reaction.user_id == Some(BOTS_OWN_SNOWFLAKE) {
            return ();
        }

        let student_emoji = ReactionType::Unicode("📚".to_string());
        let alumni_emoji = ReactionType::Unicode("🎓".to_string());
        let imposter_emoji = ReactionType::Unicode("👽".to_string());
        // try hard to avoid contention so unless a prime emoji ignore

        if reaction.emoji != student_emoji
            && reaction.emoji != alumni_emoji
            && reaction.emoji != imposter_emoji
        {
            println!("Irrelevant reaction");
            return ();
        }
        println!("Someone reacted to my message!");
        let forms: Arc<DashMap<UserId, Form, RandomState>> = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<Forms>()
                .expect("Expected Form in TypeMap.")
                .clone()
        };

        if let None = reaction.user_id {
            println!("can't find the reaction author");
            return ();
        }
        let user_id = reaction.user_id.unwrap();
        let user = match user_id.to_user(&ctx).await {
            Err(e) => {
                println!("{:?}", e);
                return ();
            }
            Ok(user) => user,
        };
        let entry = forms.get(&user_id);
        if !entry.is_none() {
            return ();
        }
        if reaction.emoji == student_emoji {
            drop(entry);
            forms.insert(user_id, Form::UnnamedStudent);
            err_say(
                user.direct_message(&ctx, |m| m.content(FULL_NAME_PLEASE))
                    .await,
            )
        } else {
            // in this case reaction.emoji can only be alumni or alien as already checked above
            drop(entry);
            forms.insert(user_id, Form::CompletedNonStudent);
            let useful_info = if reaction.emoji == alumni_emoji {
                format!("{}", Mention::from(user_id))}
            else {format!("{}", Mention::from(user_id))};

            // TODO join these futures
            err_say(VERIFICATION_CHANNEL.say(&ctx, useful_info).await);
            err_say(
                user.direct_message(&ctx, |m| m.content(JUST_FINISHED_NON_STUDENT_FORM))
                    .await,
            )
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id != BOTS_OWN_SNOWFLAKE && msg.guild_id.is_none() && (!msg.content.contains("=^..^=")) {
            message_not_self(ctx, msg).await
        }
    }
}

#[help]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let _ = plain(context, msg, args, &help_options, groups, owners).await;
    Ok(())
}

#[tokio::main] //(flavor = "current_thread")]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| {
            c.prefix("=^..^=").with_whitespace(WithWhiteSpace {
                prefixes: true,
                groups: true,
                commands: true,
            })
        })
        .help(&HELP)
        .group(&GENERAL_GROUP);
    let mut client = Client::builder("")
        .event_handler(Handler)
        .framework(framework)
        .intents(
            GatewayIntents::GUILDS
                | GatewayIntents::GUILD_MEMBERS
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::DIRECT_MESSAGES
                | GatewayIntents::GUILD_MESSAGE_REACTIONS
                | GatewayIntents::DIRECT_MESSAGE_REACTIONS,
        )
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<Forms>(Arc::new(DashMap::new()));
    }
    err_say(client.start().await);
}
