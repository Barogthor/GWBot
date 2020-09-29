use serenity::framework::standard::macros::command;
use serenity::client::{Context};
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, Args};
use crate::SkillCodeParser;
use serenity::utils::MessageBuilder;
use serenity::http::routing::RouteInfo::{CreateEmoji, GetEmoji};
use serenity::model::guild::Emoji;
use std::iter::Map;
use serenity::model::id::EmojiId;
use std::collections::HashMap;

#[command]
async fn skill(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let code_skill = args.single::<String>()?;
    let skill_record = SkillCodeParser::parse(code_skill.clone());
    println!("{:?}", skill_record);

    // let emojis = ctx.http.guild.get_guild(ctx.shard_id).await?.emojis(&ctx.http).await?;
    let channel = msg.channel_id.to_channel(&ctx).await?.guild().unwrap();

    let emojis = ctx.http.get_guild(channel.guild_id.0).await?.emojis;
    let emoji_lookup = emojis.iter()
        .map(|(id, emoji)| (emoji.name.clone(), emoji.clone()))
        .collect::<HashMap<String, Emoji>>();

    let mut response = MessageBuilder::new();
    response
        .emoji(emoji_lookup.get(&skill_record.primary_profession.to_string()).unwrap())
        .push_bold(skill_record.primary_profession.to_string())
        .push(" / ")
        .push_bold(skill_record.secondary_profession.to_string())
        .emoji(emoji_lookup.get(&skill_record.secondary_profession.to_string()).unwrap())
        .push("--")
        .push_mono(code_skill)
        .push("--")
        .push_line("");
    let mut count = skill_record.attributes.len();
    for (attr, points) in skill_record.attributes {
        count-=1;
        response.push(attr.to_string()).push(": ").push_bold(points);
        if count>0 {
            response.push(", ");
        }
    }

    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", why);
    }
    // msg.reply(ctx, code_skill).await?;

    Ok(())
}